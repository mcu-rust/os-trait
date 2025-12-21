# os-trait

[![CI](https://github.com/mcu-rust/os-trait/workflows/CI/badge.svg)](https://github.com/mcu-rust/os-trait/actions)
[![Crates.io](https://img.shields.io/crates/v/os-trait.svg)](https://crates.io/crates/os-trait)
[![Docs.rs](https://docs.rs/os-trait/badge.svg)](https://docs.rs/os-trait)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](./LICENSE)
[![Downloads](https://img.shields.io/crates/d/os-trait.svg)](https://crates.io/crates/os-trait)


**`os-trait` provides a unified trait layer for adapting multiple RTOS implementations to embedded Rust HALs.**
It makes embedded Rust code more portable, testable, and OS‑agnostic by standardizing common OS primitives such as mutexes, delays, timeouts, notifier, and thread yielding.

This crate integrates with several foundational components of the embedded Rust ecosystem:

- [`timeout-trait`](https://crates.io/crates/timeout-trait) — timeout abstractions  
- [`embedded-hal`](https://crates.io/crates/embedded-hal) — uses the `DelayNs` trait  
- [`mutex`](https://crates.io/crates/mutex) — uses `BlockingMutex` and `RawMutex`  

## 📦 Usage

```sh
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
    if t.timeout() {
        // handle timeout
    }
}

fn select_os() {
    use_os::<FakeOs>();
    use_os::<StdOs>();
}
```

## ⚙️ Cargo Features

| Feature             | Default | Description                                                                 |
|---------------------|---------|-----------------------------------------------------------------------------|
| `alloc`             | ✔️      | Enables allocation support                                                  |
| `std`               | ❌      | Enables `std` for unit testing                                              |
| `std-custom-mutex`  | ❌      | Use `BlockingMutex` instead of `std::sync::Mutex` in `std` environments     |

## 🧩 Implementing Your Own OS

Implement the `OsInterface` trait and provide:
- A `RawMutex` implementation
- A `Notifier` implementation
- A `DelayNs` implementation  
- A timeout implementation  
- A thread‑yielding function

Once implemented, your OS becomes compatible with any HAL or driver that depends on `os-trait`.

For a full implementation example, see [os_trait_impls.rs for FreeRTOS](https://github.com/mcu-rust/FreeRTOS/blob/main/freertos/src/os_trait_impls.rs).  
Basic examples are available in [os_impls.rs](src/os_impls.rs).


## 🔖 Keywords

embedded rust · rtos · hal · mutex · delay · timeout · portability · no_std · embedded-hal · traits






