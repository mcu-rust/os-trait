//! Traits used to adapter different embedded RTOS.
//! It defines a trait [`OsInterface`].
//!
//! We use the [`mutex-traits`](https://crates.io/crates/mutex-traits) crate to provide mutex functionality.
//! You can implement your own mutex by implementing the `RawMutex` trait from the `mutex-traits` crate.
//!
//! # Example
//!
//! ```
//! use os_trait::{prelude::*, FakeOs, StdOs, Duration, Timeout};
//!
//! fn use_os<OS: OsInterface>() {
//!     let mutex = OS::mutex(2);
//!
//!     let mut guard = mutex.try_lock().unwrap();
//!     assert_eq!(*guard, 2);
//!
//!     OS::yield_thread();
//!     OS::delay().delay_ms(1);
//!
//!     let mut t = Timeout::<OS>::from_millis(1);
//!     if t.timeout() {
//!         // handle timeout
//!     }
//!
//!     let (notifier, waiter) = OS::notify();
//!     assert!(notifier.notify());
//!     assert!(waiter.wait(&Duration::<OS>::from_millis(1)));
//! }
//!
//! fn select_os() {
//!     use_os::<FakeOs>();
//!     use_os::<StdOs>();
//! }
//! ```
//!
//! Use alias for convenience:
//! ```
//! use os_trait::{prelude::*, StdOs as OS, os_type_alias};
//!
//! os_type_alias!(OS);
//!
//! fn use_os_type() {
//!     let mutex = Mutex::new(2);
//!     OS::yield_thread();
//!     OS::delay().delay_ms(1);
//!
//!     let t = Timeout::from_millis(1);
//!     let dur = Duration::from_millis(1);
//!
//!     let (notifier, waiter) = OS::notify();
//!     assert!(notifier.notify());
//!     assert!(waiter.wait(&Duration::from_millis(1)));
//! }
//! ```

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
pub use timeout_trait::{self, *};

#[cfg(feature = "alloc")]
extern crate alloc;

/// The interface for different operating systems.
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
pub type Timeout<OS> = TickTimeout<<OS as OsInterface>::Instant>;
pub type Duration<OS> = TickDuration<<OS as OsInterface>::Instant>;

/// Use this macro to alias the OS-specific types for greater convenience,
/// or manually alias only the ones you need.
#[macro_export]
macro_rules! os_type_alias {
    ($YOUR_OS:ty) => {
        pub type Mutex<T> = $crate::Mutex<$YOUR_OS, T>;
        pub type Notifier = $crate::Notifier<$YOUR_OS>;
        pub type NotifyWaiter = $crate::NotifyWaiter<$YOUR_OS>;
        pub type Timeout = $crate::Timeout<$YOUR_OS>;
        pub type Duration = $crate::Duration<$YOUR_OS>;
    };
}
