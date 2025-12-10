//! See [`OsInterface`]

#![cfg_attr(not(feature = "std"), no_std)]

pub mod mutex_impls;
pub mod notifier;
pub mod notifier_impls;
pub mod os_impls;
pub mod prelude;
pub mod timeout;

pub use embedded_hal;
pub use fugit;
pub use mutex_impls::{FakeRawMutex, Mutex};
pub use mutex_traits;
pub use notifier_impls::FakeNotifier;
pub use os_impls::FakeOs;
pub use timeout::*;

use mutex_traits::{ConstInit, RawMutex};
use prelude::*;

#[cfg(not(feature = "std"))]
extern crate alloc;

/// Adapter for different operating systems.
///
/// We use the [`mutex-traits`](https://crates.io/crates/mutex-traits) crate to provide mutex functionality.
/// You need to select an appropriate mutex implementation based on your needs.
/// And you can implement your own mutex by implementing the `RawMutex` trait from the `mutex-traits` crate.
///
/// ```
/// use os_traits::{prelude::*, os_impls::{FakeOs, StdOs}};
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
    type NotifyBuilder: NotifyBuilder;

    fn yield_thread();
    fn delay() -> impl DelayNs;
    fn timeout() -> impl TimeoutNs;

    #[inline]
    fn mutex<T>(d: T) -> Mutex<Self, T> {
        Mutex::<Self, T>::new(d)
    }

    #[inline]
    fn notifier_isr() -> (impl NotifierIsr, impl NotifyWaiter) {
        Self::NotifyBuilder::build_isr()
    }

    #[inline]
    fn notifier() -> (impl Notifier, impl NotifyWaiter) {
        Self::NotifyBuilder::build()
    }
}
