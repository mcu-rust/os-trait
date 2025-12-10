use super::{
    fugit::{KilohertzU32, RateExtU32},
    prelude::*,
};
use std::time::{Duration, Instant};

/// [`TimeoutNs`] implementation.
pub struct StdTimeoutNs {}

impl TimeoutNs for StdTimeoutNs {
    fn start_ns(timeout: u32) -> impl TimeoutState {
        StdTimeoutState {
            timeout: Duration::from_nanos(timeout.into()),
            start_time: Instant::now(),
        }
    }

    fn start_us(timeout: u32) -> impl TimeoutState {
        StdTimeoutState {
            timeout: Duration::from_micros(timeout.into()),
            start_time: Instant::now(),
        }
    }

    fn start_ms(timeout: u32) -> impl TimeoutState {
        StdTimeoutState {
            timeout: Duration::from_millis(timeout.into()),
            start_time: Instant::now(),
        }
    }
}

/// [`TimeoutState`] implementation for.
pub struct StdTimeoutState {
    timeout: Duration,
    start_time: Instant,
}

impl TimeoutState for StdTimeoutState {
    #[inline]
    fn timeout(&mut self) -> bool {
        if self.start_time.elapsed() >= self.timeout {
            self.start_time += self.timeout;
            true
        } else {
            false
        }
    }

    #[inline(always)]
    fn restart(&mut self) {
        self.start_time = Instant::now();
    }
}

impl TickInstant for Instant {
    fn frequency() -> KilohertzU32 {
        1.MHz()
    }

    #[inline(always)]
    fn now() -> Self {
        Instant::now()
    }

    #[inline(always)]
    fn tick_since(self, earlier: Self) -> u32 {
        self.duration_since(earlier).as_micros() as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::timeout::tick_impl::*;
    use std::{thread::sleep, time::Duration};

    fn test_timeout<T: TimeoutNs>() {
        let mut t = T::start_ms(500);
        assert!(!t.timeout());
        sleep(Duration::from_millis(260));
        assert!(!t.timeout());
        sleep(Duration::from_millis(260));
        assert!(t.timeout());
        assert!(!t.timeout());

        t.restart();
        assert!(!t.timeout());
        sleep(Duration::from_millis(260));
        assert!(!t.timeout());
        sleep(Duration::from_millis(260));
        assert!(t.timeout());
        assert!(!t.timeout());

        assert!(T::ns_with(100, || {
            sleep(Duration::from_nanos(1));
            true
        }));
    }

    #[test]
    fn std_timeout() {
        test_timeout::<StdTimeoutNs>();
    }

    #[test]
    fn tick_timeout() {
        test_timeout::<TickTimeoutNs::<Instant>>();
    }

    #[test]
    fn tick_instant() {
        let now = <Instant as TickInstant>::now();
        sleep(Duration::from_millis(200));
        assert!(now.tick_elapsed() - 200_000 < 1000);
    }
}
