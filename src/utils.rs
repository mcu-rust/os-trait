use crate::{
    delay_impls::*,
    timeout::{tick::*, *},
    *,
};
use core::{cell::Cell, marker::PhantomData};
use fugit::MicrosDurationU32;

/// Can be used as a static builder
pub struct TickBuilder<T> {
    frequency: Cell<u32>,
    _t: PhantomData<T>,
}

unsafe impl<T: TickInstant> Sync for TickBuilder<T> {}

impl<T> TickBuilder<T>
where
    T: TickInstant,
{
    pub const fn new(frequency: u32) -> Self {
        Self {
            frequency: Cell::new(frequency),
            _t: PhantomData,
        }
    }

    pub fn set(&self, frequency: u32) {
        critical_section::with(|_| {
            self.frequency.set(frequency);
        })
    }

    pub fn timeout(&self) -> TickTimeoutNs<T> {
        TickTimeoutNs::new(self.frequency.get())
    }

    pub fn delay(&self) -> TickDelay<T> {
        TickDelay::new(self.frequency.get())
    }
}

// ------------------------------------------------------------------

pub struct PresetTickTimeout<T> {
    timeout: TickTimeoutNs<T>,
    timeout_us: u32,
}
impl<T: TickInstant> PresetTickTimeout<T> {
    pub fn new(frequency: u32, timeout: MicrosDurationU32) -> Self {
        Self {
            timeout: TickTimeoutNs::<T>::new(frequency),
            timeout_us: timeout.ticks(),
        }
    }

    pub fn start(&self) -> impl TimeoutState {
        self.timeout.start_us(self.timeout_us)
    }
}
