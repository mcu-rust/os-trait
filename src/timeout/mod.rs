pub mod delay_impls;
pub mod fake_impls;
pub mod prelude;
#[cfg(feature = "std")]
pub mod std_impls;
pub mod tick_impl;
pub mod utils;

pub use delay_impls::TickDelay;
pub use embedded_hal;
pub use fake_impls::*;
pub use fugit::{self, KilohertzU32, MicrosDurationU32};
pub use tick_impl::{TickTimeoutNs, TickTimeoutState};

pub trait TimeoutNs {
    fn start_ns(timeout: u32) -> impl TimeoutState;
    fn start_us(timeout: u32) -> impl TimeoutState;
    fn start_ms(timeout: u32) -> impl TimeoutState;

    fn ns_with(timeout: u32, mut f: impl FnMut() -> bool) -> bool {
        let mut t = Self::start_ns(timeout);
        while f() {
            if t.timeout() {
                return true;
            }
        }
        false
    }

    fn us_with(timeout: u32, mut f: impl FnMut() -> bool) -> bool {
        let mut t = Self::start_us(timeout);
        while f() {
            if t.timeout() {
                return true;
            }
        }
        false
    }

    fn ms_with(timeout: u32, mut f: impl FnMut() -> bool) -> bool {
        let mut t = Self::start_ms(timeout);
        while f() {
            if t.timeout() {
                return true;
            }
        }
        false
    }
}

pub trait TimeoutState {
    /// Check if the time limit expires.
    fn timeout(&mut self) -> bool;
    /// Reset the timeout condition.
    fn restart(&mut self);
}

pub trait TickInstant: Copy {
    fn frequency() -> KilohertzU32;
    fn now() -> Self;
    /// Returns the amount of ticks elapsed from another instant to this one.
    fn tick_since(self, earlier: Self) -> u32;
    /// Returns the amount of ticks elapsed since this instant.
    fn tick_elapsed(self) -> u32 {
        Self::now().tick_since(self)
    }
}
