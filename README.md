# os-trait
Traits used to adapter different embedded RTOS.


## Cargo Features

- `alloc`: Enabled by default.
- `std`: Used for unit test. Disabled by default.
- `std-custom-mutex`: Enable it when you want to use `BlockingMutex` instead of STD `Mutex`.

## Usage
```shell
cargo add os-trait
```

See [crate](https://crates.io/crates/os-trait)
