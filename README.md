# os-trait

[![CI](https://github.com/mcu-rust/os-trait/workflows/CI/badge.svg)](https://github.com/mcu-rust/os-trait/actions)
[![Crates.io](https://img.shields.io/crates/v/os-trait.svg)](https://crates.io/crates/os-trait)
[![Downloads](https://img.shields.io/crates/d/os-trait.svg)](https://crates.io/crates/os-trait)

Traits used to adapt different RTOSes to various HAL libraries. It relies on several important crates to achieve this goal.
- [timeout-trait](https://crates.io/crates/timeout-trait): Traits about timeout.
- [embedded-hal](https://crates.io/crates/embedded-hal): Using the trait `DelayNs` of it.
- [mutex](https://crates.io/crates/mutex): Using the struct `BlockingMutex` and the trait `RawMutex` of it.

## Cargo Features

- `alloc`: Enabled by default.
- `std`: Used for unit test. Disabled by default.
- `std-custom-mutex`: Enable it when you want to use `BlockingMutex` instead of STD `Mutex` in STD environment.

## Usage
```shell
cargo add os-trait
```

```rust
use os_trait::{prelude::*, FakeOs, StdOs};

fn use_os<OS: OsInterface>() {
    let mutex = OS::mutex(2);

    let mut guard = mutex.try_lock().unwrap();
    assert_eq!(*guard, 2);

    OS::yield_thread();

    OS::delay().delay_ms(1);

    let mut t = OS::timeout().start_ms(1);
    if t.timeout() {}
}

fn select_os() {
    use_os::<FakeOs>();
    use_os::<StdOs>();
}
```

You can find more examples about implementation and usage at [os_impls.rs](src/os_impls.rs)
