use super::{fugit::KilohertzU32, prelude::*};
use core::{cell::Cell, marker::PhantomData};

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
