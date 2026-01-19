#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]

pub mod mutex;
pub mod notifier;
pub mod notifier_impls;
pub mod os_impls;
pub mod prelude;

pub use fugit;
pub use mutex::{BlockingMutex, FakeRawMutex};
pub use mutex_traits;
pub use mutex_traits::{ConstInit, RawMutex};
pub use notifier::*;
pub use notifier_impls::*;
pub use os_impls::{FakeOs, StdOs};
pub use portable_atomic;
pub use timeout_trait::{self, *};

#[cfg(feature = "alloc")]
extern crate alloc;

/// The interface for different operating systems.
///
/// We use the [`mutex-traits`](https://crates.io/crates/mutex-traits) crate to provide mutex functionality.
/// You can implement your own mutex by implementing the `RawMutex` trait from the `mutex-traits` crate.
pub trait OsInterface: Send + Sync + Sized + 'static {
    type RawMutex: ConstInit + RawMutex;
    type Notifier: NotifierInterface;
    type NotifyWaiter: NotifyWaiterInterface<Self>;
    type Instant: TickInstant;
    type Delay: DelayNs;

    /// It's used to avoid writing `foo::<OS, _, _, _>(...)`
    const O: Self;

    fn yield_thread();

    #[inline(always)]
    fn yield_task() {
        Self::yield_thread()
    }

    fn delay() -> Self::Delay;
    fn notify() -> (Self::Notifier, Self::NotifyWaiter);

    #[inline]
    fn mutex<T>(d: T) -> Mutex<Self, T> {
        Mutex::<Self, T>::new(d)
    }
}

pub type Mutex<OS, T> = BlockingMutex<<OS as OsInterface>::RawMutex, T>;
pub type Notifier<OS> = <OS as OsInterface>::Notifier;
pub type NotifyWaiter<OS> = <OS as OsInterface>::NotifyWaiter;
pub type Instant<OS> = <OS as OsInterface>::Instant;
pub type Duration<OS> = TickDuration<Instant<OS>>;
pub type Timeout<OS> = TickTimeout<Instant<OS>>;
pub type Delay<OS> = <OS as OsInterface>::Delay;

/// Use this macro to alias the OS-specific types for greater convenience,
/// or manually alias only the ones you need.
#[macro_export]
macro_rules! os_type_alias {
    ($YOUR_OS:ty) => {
        pub type Mutex<T> = $crate::Mutex<$YOUR_OS, T>;
        pub type Notifier = $crate::Notifier<$YOUR_OS>;
        pub type NotifyWaiter = $crate::NotifyWaiter<$YOUR_OS>;
        pub type Instant = $crate::Instant<$YOUR_OS>;
        pub type Duration = $crate::Duration<$YOUR_OS>;
        pub type Timeout = $crate::Timeout<$YOUR_OS>;
        pub type Delay = $crate::Delay<$YOUR_OS>;
    };
}
