use crate::{notifier::*, *};
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

impl<OS: OsInterface> NotifyWaiter<OS> for FakeNotifier {
    fn wait(&self, _timeout: &Duration<OS>) -> bool {
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

impl<OS: OsInterface> NotifyWaiter<OS> for AtomicNotifyWaiter<OS> {
    fn wait(&self, timeout: &Duration<OS>) -> bool {
        let mut t = Timeout::<OS>::from_duration(timeout);
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

    impl NotifyWaiter<StdOs> for StdNotifyWaiter {
        fn wait(&self, timeout: &Duration<StdOs>) -> bool {
            let mut t = Timeout::<StdOs>::from_duration(timeout);
            while !t.timeout() {
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
        use std::thread;
        type OsDuration = Duration<StdOs>;

        #[test]
        fn notify() {
            let (n, w) = StdNotifier::new();
            assert!(!w.wait(&OsDuration::from_millis(1)));
            n.notify();
            assert!(w.wait(&OsDuration::from_millis(1)));

            let mut handles = vec![];

            let n2 = n.clone();

            handles.push(thread::spawn(move || {
                assert!(w.wait(&OsDuration::from_millis(2000)));
                assert!(w.wait(&OsDuration::from_millis(2000)));

                let mut i = 0;
                assert_eq!(
                    w.wait_with(&OsDuration::from_millis(100), 4, || {
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
