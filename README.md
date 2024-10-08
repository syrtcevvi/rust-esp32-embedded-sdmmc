# Esp32 and embedded-sdmmc esp-hal and esp-idf-hal examples

<a href="https://docs.rs/embedded-sdmmc/0.8.0/embedded_sdmmc/"><img src="https://img.shields.io/badge/embedded_sdmmc-0.8.0-green?style=flat&link=https://docs.rs/embedded-sdmmc/0.8.0/embedded_sdmmc/" alt="embedded_sdmmc" /></a>
<a href="https://docs.rs/embedded-hal-bus/0.2.0/embedded_hal_bus/"><img src="https://img.shields.io/badge/embedded--hal--bus-0.2.0-green?style=flat&link=https://docs.rs/embedded-hal-bus/0.2.0/embedded_hal_bus/" alt="embedded-hal-bus" /></a>
<a href="https://docs.esp-rs.org/esp-hal/esp-hal/0.20.1/esp32/esp_hal/"><img src="https://img.shields.io/badge/esp--hal-0.20.1-green?style=flat&link=https://docs.esp-rs.org/esp-hal/esp-hal/0.20.1/esp32/esp_hal/" alt="esp-hal" /></a>
<img src="https://img.shields.io/badge/esp32-purple?style=flat" alt="esp32" />

Current repo contains the examples of usage of the [embedded-sdmmc](https://crates.io/crates/embedded-sdmmc) library with esp32 with `no-std` and `std` approaches. The `no-std` example is located at [/examples/esp-hal](examples/esp-hal/README.md) and the `std` one at `/examples/esp-idf-hal` *WIP*

> Look at the [embedded_sdmmc](https://docs.rs/embedded-sdmmc/0.7.0/embedded_sdmmc/struct.SdCard.html) docs to better understand why in the `SpiDevice` the `DummyCsPin` is used rather then the actual `ps` pin (in `esp-hal` example)

> `esp-hal` provides only the `Spi`-driver and it implements only the [SpiBus](https://docs.rs/embedded-hal/1.0.0/embedded_hal/spi/trait.SpiBus.html), but the `SdCard` requires [SpiDevice](https://docs.rs/embedded-hal/1.0.0/embedded_hal/spi/trait.SpiDevice.html) trait implemented, so we need the [embedded-hal-bus](https://docs.rs/embedded-hal-bus/0.2.0/embedded_hal_bus/index.html). More info at [esp-hal](https://docs.esp-rs.org/esp-hal/esp-hal/0.18.0/esp32/esp_hal/spi/master/index.html#shared-spi-access)

## esp-hal example

Inspired by this [example](https://github.com/Nereuxofficial/esp-sdcard) but it uses older versions of the `esp-hal` and `embedded-sdmmc`

## esp-idf-hal example

*WIP*

## Running examples

Both examples were tested on the `esp-wroom-32` dev-board. [There is a good page](https://randomnerdtutorials.com/esp32-spi-communication-arduino/) with information about `ESP32 SPI` communication. Generally encouraged to read through it before running the examples for better understanding.

For the sake of brevity, `esp32` has 2 `spi` interfaces available for usage in projects: `spi2` (aka HSPI) and `spi3` (aka VSPI). I chose to use the `VSPI` interface (pins are more conveniently located), so the following pins are used:
- MOSI  (serial_out):   GPIO23
- MISO  (serial_in):    GPIO19
- SCLK  (sclk):         GPIO18
- CS    (cs):           GPIO5

> Connect the `VCC` pin of the `sd-card module` to the 5V source. In case of 3.3V the `sd` card won't respond to any command and you will get an error `TimeoutACommand`. But actually you should check the datasheet of your `sd-card module` to check the appropriate voltage.
VCC   (vcc):          5V

The `spi` bus can be used `exclusively` or can be shared among different devices. So, examples are implemented for both situations.
To run specific example move to the appropriate directory (`examples/esp-hal` for the `no-std` or `examples/esp-idf-hal` for the `std` *WIP*) and run the following command:
```
cargo run --release -F exclusive
```

or 

```
cargo run --release -F shared
```