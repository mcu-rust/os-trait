pub use crate::{
    OsInterface,
    notifier::{NotifierInterface as _, NotifyWaiterInterface as _},
    timeout_trait::prelude::*,
};

cfg_if::cfg_if! {
    if #[cfg(feature = "std")] {
        pub use std::sync::Arc;
    } else if #[cfg(feature = "alloc")] {
        pub use alloc::vec::Vec;
        pub use alloc::boxed::Box;
        pub use alloc::sync::Arc;
    }
}
