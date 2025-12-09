use super::*;

pub trait NotifierIsr: Send + Sync {
    fn notify_from_isr(&self);
}

pub trait Notifier: Send + Sync {
    fn notify(&self);
}

pub trait NotifyWaiter: Send {
    /// Wait until notified or timeout occurs.
    /// # Returns
    ///   - `true` notified
    ///   - `false` timeout occurred
    fn wait(&self, timeout: MicrosDurationU32) -> bool;
}
