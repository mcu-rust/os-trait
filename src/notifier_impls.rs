use crate::{fugit::MicrosDurationU32, notifier::*, *};
use core::{
    marker::PhantomData,
    sync::atomic::{AtomicBool, Ordering},
};

#[derive(Clone)]
pub struct FakeNotifier;

impl FakeNotifier {
    pub fn new() -> (Self, Self) {
        (Self {}, Self {})
    }
}

impl Notifier for FakeNotifier {
    fn notify(&self) -> bool {
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
    pub fn new() -> (Self, AtomicNotifyWaiter<OS>) {
        let s = Self {
            flag: Arc::new(AtomicBool::new(false)),
            _os: PhantomData,
        };
        let r = AtomicNotifyWaiter::<OS> {
            flag: Arc::clone(&s.flag),
            _os: PhantomData,
        };
        (s, r)
    }
}

impl<OS: OsInterface> Notifier for AtomicNotifier<OS> {
    fn notify(&self) -> bool {
        self.flag.store(true, Ordering::Release);
        true
    }
}

pub struct AtomicNotifyWaiter<OS> {
    flag: Arc<AtomicBool>,
    _os: PhantomData<OS>,
}

impl<OS: OsInterface> NotifyWaiter for AtomicNotifyWaiter<OS> {
    fn wait(&self, timeout: MicrosDurationU32) -> bool {
        let mut t = Timeout::<OS>::from_micros(timeout.ticks());
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

    impl Notifier for StdNotifier {
        fn notify(&self) -> bool {
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
                std::thread::sleep(std::time::Duration::from_nanos(1));
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
                assert!(w.wait(2000.millis()));
                assert!(w.wait(2000.millis()));

                let mut i = 0;
                assert_eq!(
                    w.wait_with(StdOs::O, 100.millis(), 4, || {
                        i += 1;
                        None::<()>
                    }),
                    None
                );
                assert_eq!(i, 5);
            }));

            handles.push(thread::spawn(move || {
                assert!(n.notify());
            }));

            handles.push(thread::spawn(move || {
                std::thread::sleep(std::time::Duration::from_millis(10));
                assert!(n2.notify());
            }));

            for h in handles {
                h.join().unwrap();
            }
        }
    }
}
