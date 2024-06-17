# Esp-hal embedded-sdmmc example

## Run example
This example uses two features:
- `exclusive` - use the spi with a single `SPI device` for the exclusive access to the spi bus
- `shared` - use the spi bus shared between multiple `SPI devices` (via `RefCell`)

```
cargo run --release -F exclusive
```

or 

```
cargo run --release -F shared
```
