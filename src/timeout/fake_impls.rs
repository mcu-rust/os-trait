use super::{
    fugit::{KilohertzU32, RateExtU32},
    prelude::*,
    tick_impl::{TickTimeoutNs, TickTimeoutState},
};
use core::cell::Cell;

pub type FakeTimeoutNs = TickTimeoutNs<FakeInstant>;
pub type FakeTimeoutState = TickTimeoutState<FakeInstant>;

static COUNTER: critical_section::Mutex<Cell<u32>> = critical_section::Mutex::new(Cell::new(0));

#[derive(Clone, Copy)]
pub struct FakeInstant {
    count: u32,
}

impl TickInstant for FakeInstant {
    fn frequency() -> KilohertzU32 {
        1.kHz()
    }

    fn now() -> Self {
        critical_section::with(|cs| {
            let c = COUNTER.borrow(cs).get() + 1;
            COUNTER.borrow(cs).set(c);
            Self { count: c }
        })
    }

    fn tick_since(self, earlier: Self) -> u32 {
        self.count.wrapping_sub(earlier.count)
    }
}
