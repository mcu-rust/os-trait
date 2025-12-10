pub use crate::{OsInterface, notifier::*, timeout::prelude::*};
pub use embedded_hal::delay::DelayNs;

cfg_if::cfg_if! {
    if #[cfg(feature = "std")] {
        pub use std::sync::Arc;
    } else {
        pub use alloc::vec::Vec;
        pub use alloc::boxed::Box;
        pub use alloc::sync::Arc;
    }
}
