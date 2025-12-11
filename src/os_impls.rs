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
    type NotifyBuilder = StdNotifier;
    type Timeout = StdTimeoutNs;

    fn yield_thread() {
        thread::yield_now();
    }

    fn delay() -> impl DelayNs {
        StdDelayNs {}
    }
}

// Fake -------------------------------------------------------------

pub struct FakeOs;
impl OsInterface for FakeOs {
    type RawMutex = FakeRawMutex;
    type NotifyBuilder = FakeNotifier;
    type Timeout = FakeTimeoutNs;

    fn yield_thread() {}

    fn delay() -> impl DelayNs {
        TickDelay::<FakeInstant>::default()
    }
}

// Tests ------------------------------------------------------------

#[cfg(feature = "std")]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::fugit::ExtU32;

    fn os_interface<OS: OsInterface>() {
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

        let (n, r) = OS::notifier();
        n.notify();
        assert!(r.wait(1.millis()));
    }

    #[test]
    fn select_os() {
        os_interface::<FakeOs>();
        os_interface::<StdOs>();
    }
}
