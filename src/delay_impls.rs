use super::*;
use crate::tick::TickTimeout;
use core::marker::PhantomData;
use fugit::ExtU32Ceil;

/// [`DelayNs`] implementation
pub struct TickDelay<T> {
    frequency: u32,
    _t: PhantomData<T>,
}

impl<T> TickDelay<T>
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

impl<T> DelayNs for TickDelay<T>
where
    T: TickInstant,
{
    #[inline]
    fn delay_ns(&mut self, ns: u32) {
        let mut t = TickTimeout::<T>::new_us(self.frequency, ns.nanos_at_least());
        while !t.timeout() {}
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

        #[test]
        fn delay() {
            let mut d = StdDelayNs::default();

            d.delay_ns(1_000_000);
            d.delay_us(1000);

            let t = Instant::now();
            d.delay_ms(500);
            let elapsed = t.elapsed();
            assert!(elapsed - Duration::from_millis(500) < Duration::from_millis(1));
        }
    }
}
