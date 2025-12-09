pub mod fake_impls;
#[cfg(feature = "std")]
pub mod std_impls;
pub mod tick;

pub use fake_impls::*;
pub use fugit::{ExtU32, MicrosDurationU32, MillisDurationU32, NanosDurationU32};

pub trait TimeoutBuilder {
    /// Set timeout.
    fn start_ns(&self, timeout: NanosDurationU32) -> impl Timeout;
    fn start_us(&self, timeout: MicrosDurationU32) -> impl Timeout;
    fn start_ms(&self, timeout: MillisDurationU32) -> impl Timeout;
}

pub trait Timeout {
    /// Check if the time limit expires.
    fn timeout(&mut self) -> bool;
    /// Reset the timeout condition.
    fn restart(&mut self);
}

/// The difference from [`Timeout`] is that the timeout is set when initialize.
pub trait PresetTimeoutBuilder {
    /// Start waiting.
    fn start(&self) -> impl Timeout;
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
