use crate::{
    mutex_impls::*,
    notifier_impls::*,
    prelude::*,
    timeout::{delay_impls::*, fake_impls::*},
};
cfg_if::cfg_if! {
    if #[cfg(feature = "std")] {
        use std::thread;
        use super::timeout::std_impls::*;
    }
}

// STD --------------------------------------------------------------

/// This implementation is only for unit testing.
pub struct StdOs;
#[cfg(feature = "std")]
impl OsInterface for StdOs {
    type RawMutex = FakeRawMutex;
    type NotifyBuilder = StdNotifier;

    fn yield_thread() {
        thread::yield_now();
    }

    fn delay() -> impl DelayNs {
        StdDelayNs::default()
    }

    fn timeout() -> impl TimeoutNs {
        StdTimeoutNs {}
    }
}

// Fake -------------------------------------------------------------

pub struct FakeOs;
impl OsInterface for FakeOs {
    type RawMutex = FakeRawMutex;
    type NotifyBuilder = FakeNotifier;

    fn yield_thread() {}

    fn delay() -> impl DelayNs {
        TickDelay::<FakeInstant>::new()
    }

    fn timeout() -> impl TimeoutNs {
        FakeTimeoutNs::new()
    }
}

// Tests ------------------------------------------------------------

#[cfg(feature = "std")]
#[cfg(test)]
mod tests {
    use super::*;
    use fugit::ExtU32;

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

        let (n, r) = OS::notifier_isr();
        n.notify_from_isr();
        assert!(r.wait(1.millis()));

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
