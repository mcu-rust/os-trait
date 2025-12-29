use crate::{OsInterface, Timeout, fugit::MicrosDurationU32, prelude::*};

/// This method should be able to call from task or ISR.
/// The implementation should handle the different situations.
pub trait Notifier: Send + Clone {
    fn notify(&self) -> bool;
}

pub trait NotifyWaiter: Send {
    /// Wait until notified or timeout occurs.
    /// # Returns
    ///   - `true` notified
    ///   - `false` timeout occurred
    fn wait(&self, timeout: MicrosDurationU32) -> bool;

    /// # Description
    /// Wait for a notification, but it can split the total timeout into small timeout.
    /// Your function will be called once immediately and after each small timeout.
    /// It's useful when you want to check something while it's waiting.
    ///
    /// # Parameters
    /// - `timeout`: Total timeout.
    /// - `count`: How many times to split the total timeout.
    ///   If you’re not sure, set it to `1`. Do **NOT** set it to `0`.
    /// - `f`: Your function. If it returns `Some()`, it will break out of the wait.
    ///
    /// # Returns
    /// - `None`: It's timeout.
    /// - `Some(x)`: The value returned by your function.
    ///
    /// # Note
    /// It may call your function more times than expected and wait longer than expected.
    #[inline]
    fn wait_with<OS: OsInterface, U>(
        &self,
        _os: OS,
        timeout: MicrosDurationU32,
        count: u32,
        mut f: impl FnMut() -> Option<U>,
    ) -> Option<U> {
        assert!(count > 0);
        let wait_t = MicrosDurationU32::from_ticks(timeout.ticks() / count);
        let mut t = Timeout::<OS>::from_micros(timeout.ticks());
        loop {
            if let Some(rst) = f() {
                return Some(rst);
            } else if t.timeout() {
                return None;
            }

            // TODO: let left = t.time_left(&dur);

            self.wait(wait_t);
        }
    }
}
