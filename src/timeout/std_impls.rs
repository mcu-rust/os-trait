use super::*;
use std::time::{Duration, Instant};

/// [`Timeout`] implementation for `std`.
#[derive(Default)]
pub struct StdTimeoutBuilder {}

impl TimeoutBuilder for StdTimeoutBuilder {
    fn start_ns(&self, timeout: NanosDurationU32) -> impl Timeout {
        StdTimeout {
            timeout: Duration::from_nanos(timeout.ticks().into()),
            start_time: Instant::now(),
        }
    }

    fn start_us(&self, timeout: MicrosDurationU32) -> impl Timeout {
        StdTimeout {
            timeout: Duration::from_micros(timeout.ticks().into()),
            start_time: Instant::now(),
        }
    }

    fn start_ms(&self, timeout: MillisDurationU32) -> impl Timeout {
        StdTimeout {
            timeout: Duration::from_millis(timeout.ticks().into()),
            start_time: Instant::now(),
        }
    }
}

/// [`Timeout`] implementation for `std`.
pub struct StdTimeout {
    timeout: Duration,
    start_time: Instant,
}

impl StdTimeout {
    pub fn new(timeout: MicrosDurationU32) -> Self {
        Self {
            timeout: Duration::from_micros(timeout.ticks().into()),
            start_time: Instant::now(),
        }
    }
}

impl Timeout for StdTimeout {
    #[inline]
    fn timeout(&mut self) -> bool {
        self.start_time.elapsed() >= self.timeout
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
        self.duration_since(earlier).as_nanos() as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{thread::sleep, time::Duration};

    #[test]
    fn std_waiter() {
        let timeout = StdTimeoutBuilder::default();
        let mut t = timeout.start_ms(200.millis());
        assert!(!t.timeout());
        sleep(Duration::from_millis(20));
        assert!(!t.timeout());
        sleep(Duration::from_millis(180));
        assert!(t.timeout());
        assert!(t.timeout());

        let timeout = StdTimeoutBuilder::default();
        let mut t = timeout.start_ms(500.millis());
        assert!(!t.timeout());
        sleep(Duration::from_millis(260));
        assert!(!t.timeout());
        sleep(Duration::from_millis(260));
        assert!(t.timeout());
        assert!(t.timeout());

        t.restart();
        assert!(!t.timeout());
        sleep(Duration::from_millis(260));
        assert!(!t.timeout());
        sleep(Duration::from_millis(260));
        assert!(t.timeout());
        assert!(t.timeout());
    }
}
