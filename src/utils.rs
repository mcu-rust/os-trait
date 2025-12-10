use crate::{
    timeout::{tick::*, *},
    *,
};
use core::{cell::Cell, marker::PhantomData};
use fugit::{KilohertzU32, MicrosDurationU32};

/// Can be used as a static holder
pub struct FrequencyHolder<T> {
    frequency: Cell<KilohertzU32>,
    _t: PhantomData<T>,
}

unsafe impl<T: TickInstant> Sync for FrequencyHolder<T> {}

impl<T> FrequencyHolder<T>
where
    T: TickInstant,
{
    pub const fn new(frequency: KilohertzU32) -> Self {
        Self {
            frequency: Cell::new(frequency),
            _t: PhantomData,
        }
    }

    pub fn set(&self, frequency: KilohertzU32) {
        critical_section::with(|_| {
            self.frequency.set(frequency);
        })
    }

    pub fn get(&self) -> KilohertzU32 {
        self.frequency.get()
    }
}

// ------------------------------------------------------------------

pub struct PresetTickTimeout<T> {
    timeout: TickTimeoutNs<T>,
    timeout_us: u32,
}
impl<T: TickInstant> PresetTickTimeout<T> {
    pub fn new(timeout: MicrosDurationU32) -> Self {
        Self {
            timeout: TickTimeoutNs::<T>::new(),
            timeout_us: timeout.to_micros(),
        }
    }

    pub fn start(&self) -> impl TimeoutState {
        self.timeout.start_us(self.timeout_us)
    }
}
