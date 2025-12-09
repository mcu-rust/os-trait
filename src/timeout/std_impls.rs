use super::*;
use std::time::{Duration, Instant};

/// [`TimeoutNs`] implementation.
#[derive(Default)]
pub struct StdTimeoutNs {}

impl TimeoutNs for StdTimeoutNs {
    fn start_ns(&self, timeout: u32) -> impl TimeoutState {
        StdTimeoutState {
            timeout: Duration::from_nanos(timeout.into()),
            start_time: Instant::now(),
        }
    }

    fn start_us(&self, timeout: u32) -> impl TimeoutState {
        StdTimeoutState {
            timeout: Duration::from_micros(timeout.into()),
            start_time: Instant::now(),
        }
    }

    fn start_ms(&self, timeout: u32) -> impl TimeoutState {
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
    use crate::timeout::tick::*;
    use std::{thread::sleep, time::Duration};

    fn test_timeout(timeout: impl TimeoutNs) {
        let mut t = timeout.start_ms(500);
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
    }

    #[test]
    fn std_timeout() {
        let timeout = StdTimeoutNs::default();
        test_timeout(timeout);
    }

    #[test]
    fn tick_timeout() {
        let timeout = TickTimeoutNs::<Instant>::new(1_000_000);
        test_timeout(timeout);
    }

    #[test]
    fn tick_instant() {
        let now = <Instant as TickInstant>::now();
        sleep(Duration::from_millis(200));
        assert!(now.tick_elapsed() - 200_000 < 1000);
    }
}
