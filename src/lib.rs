//! Traits used to adapter different embedded RTOS.
//! See [`OsInterface`]
//!
//! # Cargo Features
//!
//! - `std`: Used for unit test. Disabled by default.

#![cfg_attr(not(feature = "std"), no_std)]

pub mod mutex_impls;
pub mod notifier;
pub mod notifier_impls;
pub mod os_impls;
pub mod prelude;

pub use embedded_hal;
pub use fugit;
pub use mutex_impls::{FakeRawMutex, Mutex};
pub use mutex_traits;
pub use mutex_traits::{ConstInit, RawMutex};
pub use notifier_impls::*;
pub use os_impls::{FakeOs, StdOs};
pub use timeout_trait::{self, *};

use crate::prelude::*;

#[cfg(feature = "alloc")]
extern crate alloc;

/// Adapter for different operating systems.
///
/// We use the [`mutex-traits`](https://crates.io/crates/mutex-traits) crate to provide mutex functionality.
/// You need to select an appropriate mutex implementation based on your needs.
/// And you can implement your own mutex by implementing the `RawMutex` trait from the `mutex-traits` crate.
///
/// ```
/// use os_trait::{prelude::*, FakeOs, StdOs};
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
pub trait OsInterface: Send + Sync + 'static {
    type RawMutex: ConstInit + RawMutex;
    type Notifier: Notifier;
    type NotifyWaiter: NotifyWaiter;
    type Timeout: TimeoutNs;

    /// Used for generics functions.
    const O: Self;

    fn yield_thread();
    fn delay() -> impl DelayNs;
    fn notify() -> (Self::Notifier, Self::NotifyWaiter);

    #[inline]
    fn mutex<T>(d: T) -> Mutex<Self, T> {
        Mutex::<Self, T>::new(d)
    }
}
