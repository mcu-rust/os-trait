use crate::{
    mutex_impls::*,
    notifier_impls::*,
    prelude::*,
    timeout_trait::{delay_impls::*, fake_impls::*},
};
cfg_if::cfg_if! {
    if #[cfg(feature = "std")] {
        use std::thread;
        use crate::timeout_trait::std_impls::*;
    }
}

// STD --------------------------------------------------------------

/// This implementation is only for unit testing.
pub struct StdOs;
#[cfg(feature = "std")]
impl OsInterface for StdOs {
    type RawMutex = FakeRawMutex;
    type Notifier = StdNotifier;
    type NotifyWaiter = StdNotifyWaiter;
    type Timeout = StdTimeoutNs;
    type TimeoutState = StdTimeoutState;

    const O: Self = Self {};

    #[inline]
    fn yield_thread() {
        thread::yield_now();
    }

    #[inline]
    fn delay() -> impl DelayNs {
        StdDelayNs {}
    }

    #[inline]
    fn notify() -> (Self::Notifier, Self::NotifyWaiter) {
        StdNotifier::new()
    }
}

// Fake -------------------------------------------------------------

pub struct FakeOs;
impl OsInterface for FakeOs {
    type RawMutex = FakeRawMutex;
    type Notifier = FakeNotifier;
    type NotifyWaiter = FakeNotifier;
    type Timeout = FakeTimeoutNs;
    type TimeoutState = FakeTimeoutState;

    const O: Self = Self {};

    #[inline]
    fn yield_thread() {}

    #[inline]
    fn delay() -> impl DelayNs {
        TickDelay::<FakeInstant>::default()
    }

    #[inline]
    fn notify() -> (Self::Notifier, Self::NotifyWaiter) {
        FakeNotifier::new()
    }
}

// Tests ------------------------------------------------------------

#[cfg(feature = "std")]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::fugit::ExtU32;

    struct OsUser<OS: OsInterface> {
        notifier: OS::Notifier,
        waiter: OS::NotifyWaiter,
        mutex: Mutex<OS, u8>,
        interval: OS::TimeoutState,
    }

    impl<OS: OsInterface> OsUser<OS> {
        fn new() -> Self {
            let (notifier, waiter) = OS::notify();
            Self {
                notifier,
                waiter,
                mutex: OS::mutex(1),
                interval: OS::Timeout::start_ms(1),
            }
        }

        fn use_os(&mut self) {
            let mutex = OS::mutex(0);

            let mut guard = mutex.try_lock().unwrap();
            assert_eq!(*guard, 0);
            *guard = 4;
            drop(guard);

            mutex
                .try_with_lock(|data| {
                    assert_eq!(*data, 4);
                    *data = 5;
                })
                .unwrap();

            OS::yield_thread();
            OS::delay().delay_ms(1);

            let _os = OS::O;

            assert!(self.notifier.notify());
            assert!(self.waiter.wait(1.millis()));

            let mut d = self.mutex.lock();
            *d = 2;

            self.interval.timeout();
        }
    }

    #[test]
    fn select_os() {
        let mut user = OsUser::<FakeOs>::new();
        user.use_os();
        let mut user = OsUser::<StdOs>::new();
        user.use_os();
    }
}
