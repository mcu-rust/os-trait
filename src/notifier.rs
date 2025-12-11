use crate::fugit::MicrosDurationU32;

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

    /// # Parameters
    /// - `timeout` Total timeout.
    /// - `count` How many times to split the total timeout.
    ///   Your function will be called after each small timeout.
    ///   It's useful when you want to check something while it's waiting.
    ///   If you’re not sure, set it to `1`. Do **NOT** set it to `0`.
    /// - `f`: Your function. If it returns `Some()`, it will break out of the wait.
    #[inline]
    fn wait_with<U>(
        &self,
        timeout: MicrosDurationU32,
        count: u32,
        mut f: impl FnMut() -> Option<U>,
    ) -> Option<U> {
        assert!(count > 0);
        let t = MicrosDurationU32::from_ticks(timeout.ticks() / count);
        let mut i = 0;
        loop {
            if let Some(rst) = f() {
                return Some(rst);
            }

            i += 1;
            if i > count {
                return None;
            }
            self.wait(t);
        }
    }
}
