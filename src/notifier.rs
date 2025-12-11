use crate::fugit::MicrosDurationU32;

pub trait NotifyBuilder {
    fn build() -> (impl Notifier, impl NotifyWaiter);
}

/// This method should be able to call from task or ISR.
/// The implementation should handle the different situations.
pub trait Notifier: Send + Sync {
    fn notify(&self) -> bool;
}

pub trait NotifyWaiter: Send {
    /// Wait until notified or timeout occurs.
    /// # Returns
    ///   - `true` notified
    ///   - `false` timeout occurred
    fn wait(&self, timeout: MicrosDurationU32) -> bool;
}
