use crate::{
    mutex::FakeRawMutex,
    notifier_impls::*,
    prelude::*,
    timeout_trait::{delay::*, fake_impls::*},
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
    type Instant = StdTickInstant;
    type Delay = StdDelayNs;

    const O: Self = Self {};

    #[inline]
    fn yield_thread() {
        thread::yield_now();
    }

    #[inline]
    fn delay() -> Self::Delay {
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
    type Instant = FakeTickInstant;
    type Delay = TickDelay<FakeTickInstant>;

    const O: Self = Self {};

    #[inline]
    fn yield_thread() {}

    #[inline]
    fn delay() -> Self::Delay {
        TickDelay::<FakeTickInstant>::default()
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
    use crate::{Duration, Mutex, Timeout};

    struct OsUser<OS: OsInterface> {
        notifier: OS::Notifier,
        waiter: OS::NotifyWaiter,
        mutex: Mutex<OS, u8>,
        interval: Timeout<OS>,
    }

    impl<OS: OsInterface> OsUser<OS> {
        fn new() -> Self {
            let (notifier, waiter) = OS::notify();
            Self {
                notifier,
                waiter,
                mutex: OS::mutex(1),
                interval: Timeout::<OS>::from_millis(1),
            }
        }

        fn use_os(&mut self) {
            let mutex = Mutex::<OS, _>::new(0);

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

            assert!(self.notifier.notify());
            assert!(self.waiter.wait(&Duration::<OS>::from_millis(1)));

            let mut d = self.mutex.lock();
            *d = 2;

            if self.interval.timeout() {}
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

#[allow(dead_code)]
#[cfg(feature = "std")]
#[cfg(test)]
mod tests_end_type {
    use crate::{StdOs as OS, os_type_alias, prelude::*};

    os_type_alias!(OS);

    struct EndUser {
        notifier: Notifier,
        waiter: NotifyWaiter,
        mutex: Mutex<u8>,
        interval: Timeout,
        dur: Duration,
    }

    impl EndUser {
        pub fn new() -> Self {
            let (notifier, waiter) = OS::notify();
            Self {
                notifier,
                waiter,
                mutex: Mutex::new(1),
                interval: Timeout::from_millis(1),
                dur: Duration::from_millis(1),
            }
        }
    }
}
