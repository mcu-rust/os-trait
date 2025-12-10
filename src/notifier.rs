pub use fugit::MicrosDurationU32;

pub trait NotifyBuilder {
    fn build() -> (impl Notifier, impl NotifyWaiter);
    fn build_isr() -> (impl NotifierIsr, impl NotifyWaiter);
}

pub trait NotifierIsr: Send + Clone {
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
