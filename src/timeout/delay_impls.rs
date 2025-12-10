use super::{embedded_hal::delay::DelayNs, prelude::*, tick_impl::TickTimeoutNs};
use core::marker::PhantomData;

/// [`DelayNs`] implementation
pub struct TickDelay<T> {
    _t: PhantomData<T>,
}

impl<T> TickDelay<T>
where
    T: TickInstant,
{
    pub fn new() -> Self {
        Self { _t: PhantomData }
    }
}

impl<T> DelayNs for TickDelay<T>
where
    T: TickInstant,
{
    #[inline]
    fn delay_ns(&mut self, ns: u32) {
        let mut t = TickTimeoutNs::<T>::start_ns(ns);
        while !t.timeout() {
            // For unit test
            #[cfg(feature = "std")]
            std::thread::sleep(std::time::Duration::from_nanos(1));
        }
    }
}

#[cfg(feature = "std")]
pub use for_std::*;
#[cfg(feature = "std")]
mod for_std {
    use super::*;
    use std::time::Duration;

    #[derive(Default)]
    pub struct StdDelayNs;

    impl DelayNs for StdDelayNs {
        #[inline]
        fn delay_ns(&mut self, ns: u32) {
            std::thread::sleep(Duration::from_nanos(ns.into()))
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::time::{Duration, Instant};

        fn test_delay(mut d: impl DelayNs) {
            let t = Instant::now();
            d.delay_ns(200_000_000);
            let elapsed = t.elapsed();
            assert!(elapsed - Duration::from_millis(200) < Duration::from_millis(100));

            let t = Instant::now();
            d.delay_us(200_000);
            let elapsed = t.elapsed();
            assert!(elapsed - Duration::from_millis(200) < Duration::from_millis(100));

            let t = Instant::now();
            d.delay_ms(500);
            let elapsed = t.elapsed();
            assert!(elapsed - Duration::from_millis(500) < Duration::from_millis(100));
        }

        #[test]
        fn std_delay() {
            let d = StdDelayNs::default();
            test_delay(d);
        }

        #[test]
        fn tick_delay() {
            let d = TickDelay::<Instant>::new();
            test_delay(d);
        }
    }
}
