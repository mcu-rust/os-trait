pub use fugit::MicrosDurationU32;

pub trait NotifierIsr: Send + Sync {
    fn notify_from_isr(&self) -> bool;
}

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
