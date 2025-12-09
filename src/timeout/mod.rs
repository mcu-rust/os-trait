pub mod fake_impls;
#[cfg(feature = "std")]
pub mod std_impls;
pub mod tick;

pub use fake_impls::*;

pub trait TimeoutNs {
    /// Set timeout.
    fn start_ns(&self, timeout: u32) -> impl TimeoutState;
    fn start_us(&self, timeout: u32) -> impl TimeoutState;
    fn start_ms(&self, timeout: u32) -> impl TimeoutState;
}

pub trait TimeoutState {
    /// Check if the time limit expires.
    fn timeout(&mut self) -> bool;
    /// Reset the timeout condition.
    fn restart(&mut self);
}

pub trait TickInstant: Copy {
    fn now() -> Self;
    /// Returns the amount of ticks elapsed from another instant to this one.
    fn tick_since(self, earlier: Self) -> u32;
    /// Returns the amount of ticks elapsed since this instant.
    fn tick_elapsed(self) -> u32 {
        Self::now().tick_since(self)
    }
}
