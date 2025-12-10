use super::prelude::*;
use core::marker::PhantomData;

pub struct TickTimeoutNs<T> {
    _t: PhantomData<T>,
}

impl<T> TimeoutNs for TickTimeoutNs<T>
where
    T: TickInstant,
{
    #[inline]
    fn start_ns(timeout: u32) -> impl TimeoutState {
        TickTimeoutState::<T>::new_ns(timeout)
    }

    #[inline]
    fn start_us(timeout: u32) -> impl TimeoutState {
        TickTimeoutState::<T>::new_us(timeout)
    }

    #[inline]
    fn start_ms(timeout: u32) -> impl TimeoutState {
        TickTimeoutState::<T>::new_ms(timeout)
    }
}

pub struct TickTimeoutState<T: TickInstant> {
    tick: T,
    timeout_tick: u32,
    elapsed_tick: u32,
    round: u32,
    elapsed_round: u32,
}

impl<T> TickTimeoutState<T>
where
    T: TickInstant,
{
    pub fn new_ns(timeout: u32) -> Self {
        let ns = timeout as u64;
        let timeout_tick = (ns * T::frequency().to_kHz() as u64).div_ceil(1_000_000);
        Self::new(timeout_tick)
    }

    pub fn new_us(timeout: u32) -> Self {
        let us = timeout as u64;
        let timeout_tick = (us * T::frequency().to_kHz() as u64).div_ceil(1_000);
        Self::new(timeout_tick)
    }

    pub fn new_ms(timeout: u32) -> Self {
        let ms = timeout as u64;
        let frequency = T::frequency().to_kHz() as u64;
        let timeout_tick = ms * frequency;
        Self::new(timeout_tick)
    }

    fn new(mut timeout_tick: u64) -> Self {
        let mut round = 1;
        while timeout_tick > (u32::MAX >> 1) as u64 {
            if (timeout_tick | 1) == 1 {
                timeout_tick += 0b10;
            }
            timeout_tick >>= 1;
            round <<= 1;
        }

        Self {
            tick: T::now(),
            timeout_tick: timeout_tick as u32,
            elapsed_tick: 0,
            round,
            elapsed_round: 0,
        }
    }
}

impl<T> TimeoutState for TickTimeoutState<T>
where
    T: TickInstant,
{
    /// Can be reused without calling `restart()`.
    #[inline]
    fn timeout(&mut self) -> bool {
        let now = T::now();
        self.elapsed_tick = self.elapsed_tick.saturating_add(now.tick_since(self.tick));
        self.tick = now;

        if self.elapsed_tick >= self.timeout_tick {
            self.elapsed_tick -= self.timeout_tick;
            self.elapsed_round += 1;
            if self.elapsed_round >= self.round {
                self.elapsed_round -= self.round;
                return true;
            }
        }
        false
    }

    #[inline(always)]
    fn restart(&mut self) {
        self.tick = T::now();
        self.elapsed_tick = 0;
        self.elapsed_round = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::cell::Cell;
    use fugit::{ExtU32, KilohertzU32, NanosDurationU32, RateExtU32};

    static TICK_SOURCE: critical_section::Mutex<Cell<u32>> =
        critical_section::Mutex::new(Cell::new(0));

    #[derive(Clone, Copy)]
    pub struct MockInstant {
        tick: u32,
    }

    impl MockInstant {
        fn add_time(t: NanosDurationU32) {
            let tick = t.to_nanos();
            critical_section::with(|cs| {
                let v = TICK_SOURCE.borrow(cs).get().wrapping_add(tick);
                TICK_SOURCE.borrow(cs).set(v);
            })
        }
    }

    impl TickInstant for MockInstant {
        fn frequency() -> KilohertzU32 {
            1000.MHz()
        }

        fn now() -> Self {
            critical_section::with(|cs| Self {
                tick: TICK_SOURCE.borrow(cs).get(),
            })
        }

        fn tick_since(self, earlier: Self) -> u32 {
            self.tick.wrapping_sub(earlier.tick)
        }
    }

    #[test]
    fn tick_timeout() {
        let now = MockInstant::now();
        assert_eq!(now.tick_elapsed(), 0);
        MockInstant::add_time(10.nanos());
        assert_eq!(now.tick_elapsed(), 10);
        MockInstant::add_time(1.millis());
        assert_eq!(now.tick_elapsed(), 1_000_010);
        MockInstant::add_time(10.micros());
        let now2 = MockInstant::now();
        assert_eq!(now2.tick_since(now), 1_010_010);
        MockInstant::add_time(10.micros());
        let now3 = MockInstant::now();
        assert_eq!(now3.tick_since(now2), 10_000);

        let mut t = TickTimeoutNs::<MockInstant>::start_ns(100);
        assert!(!t.timeout());
        MockInstant::add_time(10.nanos());
        assert!(!t.timeout());
        MockInstant::add_time(90.nanos());
        assert!(t.timeout());
        assert!(!t.timeout());
        MockInstant::add_time(100.nanos());
        assert!(t.timeout());

        MockInstant::add_time(90.nanos());
        assert!(!t.timeout());
        t.restart();
        MockInstant::add_time(90.nanos());
        assert!(!t.timeout());
        MockInstant::add_time(10.nanos());
        assert!(t.timeout());

        let mut t = TickTimeoutNs::<MockInstant>::start_us(100);
        assert!(!t.timeout());
        MockInstant::add_time(10.micros());
        assert!(!t.timeout());
        MockInstant::add_time(90.micros());
        assert!(t.timeout());
        assert!(!t.timeout());
        MockInstant::add_time(100.micros());
        assert!(t.timeout());

        MockInstant::add_time(90.micros());
        assert!(!t.timeout());
        t.restart();
        MockInstant::add_time(90.micros());
        assert!(!t.timeout());
        MockInstant::add_time(10.micros());
        assert!(t.timeout());

        let mut t = TickTimeoutNs::<MockInstant>::start_ms(100);
        assert!(!t.timeout());
        MockInstant::add_time(10.millis());
        assert!(!t.timeout());
        MockInstant::add_time(90.millis());
        assert!(t.timeout());
        assert!(!t.timeout());
        MockInstant::add_time(100.millis());
        assert!(t.timeout());

        MockInstant::add_time(90.millis());
        assert!(!t.timeout());
        t.restart();
        MockInstant::add_time(90.millis());
        assert!(!t.timeout());
        MockInstant::add_time(10.millis());
        assert!(t.timeout());

        let mut count = 0;
        assert!(TickTimeoutNs::<MockInstant>::ns_with(100, || {
            MockInstant::add_time(10.nanos());
            count += 1;
            true
        }));
        assert_eq!(count, 10);

        let t = TickTimeoutState::<MockInstant>::new_us(40_000_000);
        assert_eq!(t.round, 32);
        assert_eq!(t.timeout_tick, 1_250_000_000);

        let mut t = TickTimeoutState::<MockInstant>::new_ms(40_000);
        assert_eq!(t.round, 32);
        assert_eq!(t.timeout_tick, 1_250_000_000);

        assert!(!t.timeout());

        for _ in 0..40 {
            MockInstant::add_time(999.millis());
            assert!(!t.timeout());
        }
        assert_eq!(t.elapsed_round, 31);
        assert!(t.elapsed_tick > 0);
        MockInstant::add_time(100.millis());
        assert!(t.timeout());
        assert_eq!(t.elapsed_round, 0);
        assert!(!t.timeout());

        for _ in 0..39 {
            MockInstant::add_time(999.millis());
            assert!(!t.timeout());
        }
        t.restart();
        for _ in 0..40 {
            MockInstant::add_time(999.millis());
            assert!(!t.timeout());
        }
        MockInstant::add_time(100.millis());
        assert!(t.timeout());
    }
}

#[cfg(test)]
mod tests_fugit {
    use core::ops::Div;
    use fugit::{
        ExtU32, ExtU32Ceil, KilohertzU32, MicrosDurationU32, MillisDurationU32, RateExtU32,
    };

    #[test]
    fn duration_tick() {
        assert_eq!(1 / 1000, 0);
        assert_eq!(1_u32.div(1000), 0);
        assert_eq!(1_u32.div_ceil(1000), 1);

        let dur: MicrosDurationU32 = 1.micros();
        assert_eq!(dur.ticks(), 1);

        let dur: MicrosDurationU32 = 1.millis();
        assert_eq!(dur.ticks(), 1000);
        assert_eq!(dur.to_millis(), 1);

        let dur: MillisDurationU32 = 1.millis();
        assert_eq!(dur.ticks(), 1);
        assert_eq!(dur.to_micros(), 1000);

        let dur: MillisDurationU32 = 1.micros();
        assert_eq!(dur.ticks(), 0);

        let dur: MillisDurationU32 = 1.micros_at_least();
        assert_eq!(dur.ticks(), 1);

        let dur: MicrosDurationU32 = 1.micros();
        assert_eq!(dur.to_millis(), 0);
        let dur: MillisDurationU32 = dur.ticks().micros_at_least();
        assert_eq!(dur.ticks(), 1);
    }

    #[test]
    fn rate_tick() {
        let r: KilohertzU32 = 1.MHz();
        assert_eq!(r.raw(), 1_000);
        let f: u32 = r.to_Hz();
        assert_eq!(f, 1_000_000);
    }
}
