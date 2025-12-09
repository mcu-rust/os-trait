use super::*;
use core::marker::PhantomData;

pub struct TickTimeoutNs<T> {
    frequency: u32,
    _t: PhantomData<T>,
}

impl<T> TickTimeoutNs<T>
where
    T: TickInstant,
{
    pub fn new(frequency: u32) -> Self {
        Self {
            frequency,
            _t: PhantomData,
        }
    }
}

impl<T> TimeoutNs for TickTimeoutNs<T>
where
    T: TickInstant,
{
    #[inline]
    fn start_ns(&self, timeout: u32) -> impl TimeoutState {
        TickTimeoutState::<T>::new_ns(self.frequency, timeout)
    }

    #[inline]
    fn start_us(&self, timeout: u32) -> impl TimeoutState {
        TickTimeoutState::<T>::new_us(self.frequency, timeout)
    }

    #[inline]
    fn start_ms(&self, timeout: u32) -> impl TimeoutState {
        TickTimeoutState::<T>::new_ms(self.frequency, timeout)
    }
}

pub struct TickTimeoutState<T: TickInstant> {
    tick: T,
    timeout_tick: u32,
    elapsed_tick: u32,
}

impl<T> TickTimeoutState<T>
where
    T: TickInstant,
{
    pub fn new_ns(frequency: u32, timeout: u32) -> Self {
        let ns = timeout as u64;
        let timeout_tick = (ns * frequency as u64).div_ceil(1_000_000_000);
        assert!(timeout_tick <= u32::MAX as u64);
        Self {
            tick: T::now(),
            timeout_tick: timeout_tick as u32,
            elapsed_tick: 0,
        }
    }

    pub fn new_us(frequency: u32, timeout: u32) -> Self {
        let us = timeout;
        let timeout_tick = if frequency >= 1_000_000 {
            us.checked_mul(frequency / 1_000_000).unwrap()
        } else if frequency >= 1_000 {
            us.checked_mul(frequency / 1_000).unwrap().div_ceil(1_000)
        } else {
            us.checked_mul(frequency).unwrap().div_ceil(1_000_000)
        };

        Self {
            tick: T::now(),
            timeout_tick,
            elapsed_tick: 0,
        }
    }

    pub fn new_ms(frequency: u32, timeout: u32) -> Self {
        let ms = timeout;
        let timeout_tick = if frequency >= 1_000 {
            ms.checked_mul(frequency / 1_000).unwrap()
        } else {
            ms.checked_mul(frequency).unwrap().div_ceil(1_000)
        };

        Self {
            tick: T::now(),
            timeout_tick,
            elapsed_tick: 0,
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
        self.elapsed_tick = self.elapsed_tick.add_u32(now.tick_since(self.tick));
        self.tick = now;

        if self.elapsed_tick >= self.timeout_tick {
            self.elapsed_tick -= self.timeout_tick;
            true
        } else {
            false
        }
    }

    #[inline(always)]
    fn restart(&mut self) {
        self.tick = T::now();
        self.elapsed_tick = 0;
    }
}

pub trait Num: Sized + Copy + core::cmp::Ord + core::ops::SubAssign {
    const ZERO: Self;
    fn add_u32(self, v: u32) -> Self;
}

impl Num for u32 {
    const ZERO: Self = 0;
    fn add_u32(self, v: u32) -> Self {
        self.saturating_add(v)
    }
}

impl Num for u64 {
    const ZERO: Self = 0u64;
    fn add_u32(self, v: u32) -> Self {
        self.saturating_add(v as u64)
    }
}

#[cfg(test)]
mod tests {
    use core::ops::Div;
    use fugit::{ExtU32, ExtU32Ceil, MicrosDurationU32, MillisDurationU32};

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

        let dur: MillisDurationU32 = 1.micros();
        assert_eq!(dur.ticks(), 0);

        let dur: MillisDurationU32 = 1.micros_at_least();
        assert_eq!(dur.ticks(), 1);

        let dur: MicrosDurationU32 = 1.micros();
        assert_eq!(dur.to_millis(), 0);
        let dur: MillisDurationU32 = dur.ticks().micros_at_least();
        assert_eq!(dur.ticks(), 1);
    }
}
