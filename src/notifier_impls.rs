use crate::{fugit::MicrosDurationU32, notifier::*, *};
use core::{
    marker::PhantomData,
    sync::atomic::{AtomicBool, Ordering},
};

#[derive(Default, Clone)]
pub struct FakeNotifier;

impl NotifyBuilder for FakeNotifier {
    fn build() -> (impl Notifier, impl NotifyWaiter) {
        (Self {}, Self {})
    }

    fn build_isr() -> (impl NotifierIsr, impl NotifyWaiter) {
        (Self {}, Self {})
    }
}

impl Notifier for FakeNotifier {
    fn notify(&self) -> bool {
        true
    }
}

impl NotifierIsr for FakeNotifier {
    fn notify_from_isr(&self) -> bool {
        true
    }
}

impl NotifyWaiter for FakeNotifier {
    fn wait(&self, _timeout: MicrosDurationU32) -> bool {
        true
    }
}

// ------------------------------------------------------------------

/// This [`Notifier`] implementation is for unit test
pub struct AtomicNotifier<OS> {
    flag: Arc<AtomicBool>,
    _os: PhantomData<OS>,
}

impl<OS: OsInterface> Clone for AtomicNotifier<OS> {
    fn clone(&self) -> Self {
        Self {
            flag: Arc::clone(&self.flag),
            _os: PhantomData,
        }
    }
}

impl<OS: OsInterface> AtomicNotifier<OS> {
    pub fn new() -> (Self, impl NotifyWaiter) {
        let s = Self {
            flag: Arc::new(AtomicBool::new(false)),
            _os: PhantomData,
        };
        let r = AtomicNotifyReceiver::<OS> {
            flag: Arc::clone(&s.flag),
            _os: PhantomData,
        };
        (s, r)
    }
}

impl<OS: OsInterface> NotifyBuilder for AtomicNotifier<OS> {
    fn build() -> (impl Notifier, impl NotifyWaiter) {
        Self::new()
    }

    fn build_isr() -> (impl NotifierIsr, impl NotifyWaiter) {
        Self::new()
    }
}

impl<OS: OsInterface> Notifier for AtomicNotifier<OS> {
    fn notify(&self) -> bool {
        self.flag.store(true, Ordering::Release);
        true
    }
}

impl<OS: OsInterface> NotifierIsr for AtomicNotifier<OS> {
    fn notify_from_isr(&self) -> bool {
        self.flag.store(true, Ordering::Release);
        true
    }
}

pub struct AtomicNotifyReceiver<OS> {
    flag: Arc<AtomicBool>,
    _os: PhantomData<OS>,
}

impl<OS: OsInterface> NotifyWaiter for AtomicNotifyReceiver<OS> {
    fn wait(&self, timeout: MicrosDurationU32) -> bool {
        let mut t = OS::Timeout::start_us(timeout.to_micros());
        while !t.timeout() {
            if self
                .flag
                .compare_exchange(true, false, Ordering::SeqCst, Ordering::Acquire)
                .is_ok()
            {
                return true;
            }
            OS::yield_thread();
        }
        false
    }
}

// ------------------------------------------------------------------

#[cfg(feature = "std")]
pub use std_impl::*;
#[cfg(feature = "std")]
mod std_impl {
    use super::*;
    use std::sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    };
    use std::time::Instant;

    /// This implementation is only for unit testing.
    #[derive(Clone)]
    pub struct StdNotifier {
        flag: Arc<AtomicBool>,
    }

    impl StdNotifier {
        pub fn new() -> (Self, StdNotifyWaiter) {
            let s = Self {
                flag: Arc::new(AtomicBool::new(false)),
            };
            let r = StdNotifyWaiter {
                flag: Arc::clone(&s.flag),
            };
            (s, r)
        }
    }

    impl NotifyBuilder for StdNotifier {
        fn build() -> (impl Notifier, impl NotifyWaiter) {
            Self::new()
        }

        fn build_isr() -> (impl NotifierIsr, impl NotifyWaiter) {
            Self::new()
        }
    }

    impl Notifier for StdNotifier {
        fn notify(&self) -> bool {
            self.flag.store(true, Ordering::Release);
            true
        }
    }

    impl NotifierIsr for StdNotifier {
        fn notify_from_isr(&self) -> bool {
            self.flag.store(true, Ordering::Release);
            true
        }
    }

    /// This implementation is only for unit testing.
    pub struct StdNotifyWaiter {
        flag: Arc<AtomicBool>,
    }

    impl NotifyWaiter for StdNotifyWaiter {
        fn wait(&self, timeout: MicrosDurationU32) -> bool {
            let now = Instant::now();
            while now.elapsed().as_micros() < timeout.to_micros().into() {
                if self
                    .flag
                    .compare_exchange(true, false, Ordering::SeqCst, Ordering::Acquire)
                    .is_ok()
                {
                    return true;
                }
                std::thread::yield_now();
            }
            false
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use fugit::ExtU32;
        use std::thread;

        #[test]
        fn notify() {
            let (n, w) = StdNotifier::new();
            assert!(!w.wait(1.millis()));
            n.notify();
            assert!(w.wait(1.millis()));

            let mut handles = vec![];

            let n2 = n.clone();

            handles.push(thread::spawn(move || {
                assert!(w.wait(500.millis()));
                assert!(w.wait(500.millis()));
            }));

            handles.push(thread::spawn(move || {
                assert!(n.notify());
            }));

            handles.push(thread::spawn(move || {
                assert!(n2.notify());
            }));

            for h in handles {
                h.join().unwrap();
            }
        }
    }
}
