//! See [`OsInterface`]

#![cfg_attr(not(feature = "std"), no_std)]

pub mod delay_impls;
pub mod mutex_impls;
pub mod notifier;
pub mod notifier_impls;
pub mod os_impls;
pub mod timeout;

pub use mutex_impls::*;
pub use notifier::*;
pub use notifier_impls::*;
pub use os_impls::*;
pub use timeout::*;

pub use embedded_hal::{self, delay::DelayNs};
pub use fugit::{self, ExtU32, MicrosDurationU32};

use mutex_traits::{ConstInit, RawMutex};

#[cfg(not(feature = "std"))]
extern crate alloc;

/// Adapter for different operating systems.
///
/// We use the [`mutex-traits`](https://crates.io/crates/mutex-traits) crate to provide mutex functionality.
/// You need to select an appropriate mutex implementation based on your needs.
/// And you can implement your own mutex by implementing the `RawMutex` trait from the `mutex-traits` crate.
///
/// ```
/// use os_traits::*;
///
/// fn os_interface<OS: OsInterface>() {
///     let mutex = OS::mutex(2);
///
///     let mut guard = mutex.try_lock().unwrap();
///     assert_eq!(*guard, 2);
///
///     OS::yield_thread();
/// }
///
/// fn select_os() {
///     os_interface::<FakeOs>();
///     os_interface::<StdOs>();
/// }
/// ```
pub trait OsInterface: Send + Sync {
    type RawMutex: ConstInit + RawMutex;

    #[inline]
    fn mutex<T>(d: T) -> BlockingMutex<Self::RawMutex, T> {
        BlockingMutex::new(d)
    }

    fn yield_thread();
    fn delay() -> impl DelayNs;
    fn start_timeout(dur: MicrosDurationU32) -> impl Timeout;
    fn notifier_isr() -> (impl NotifierIsr, impl NotifyWaiter);
    fn notifier() -> (impl Notifier, impl NotifyWaiter);
}
