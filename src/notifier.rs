use crate::{Duration, OsInterface, Timeout};

/// This method should be able to call from task or ISR.
/// The implementation should handle the different situations.
pub trait NotifierInterface: Send + Clone {
    fn notify(&self) -> bool;
}

pub trait NotifyWaiterInterface<OS: OsInterface>: Send {
    /// Wait until notified or timeout occurs.
    /// # Returns
    ///   - `true` notified
    ///   - `false` timeout occurred
    fn wait(&self, timeout: &Duration<OS>) -> bool;

    /// # Description
    /// Wait for a notification, and call your function to check anything you want
    /// when receiving a notification. You can control whether or not to continue
    /// waiting for the next notification.
    ///
    /// # Parameters
    /// - `timeout`: Total timeout.
    /// - `f`: Your function. If it returns `Some(x)`, it will break out of the wait.
    ///
    /// # Returns
    /// - `None`: It's timeout.
    /// - `Some(x)`: The value returned by your function.
    #[inline]
    fn wait_with<U>(&self, timeout: &Duration<OS>, mut f: impl FnMut() -> Option<U>) -> Option<U> {
        let mut wait_time = timeout.clone();
        let mut t = Timeout::<OS>::from(timeout);
        loop {
            if let Some(rst) = f() {
                return Some(rst);
            }
            if t.timeout() {
                return None;
            }

            let left_time = t.time_left();
            if left_time < wait_time {
                wait_time = left_time;
            }
            self.wait(&wait_time);
        }
    }
}
