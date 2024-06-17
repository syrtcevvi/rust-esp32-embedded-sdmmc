#![no_std]
#![no_main]

use core::cell::RefCell;

use embedded_hal_bus::spi::{ExclusiveDevice, RefCellDevice};
use embedded_sdmmc::{sdcard::DummyCsPin, SdCard};
use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl,
    delay::Delay,
    gpio::Level,
    gpio::{Io, Output},
    peripherals::Peripherals,
    prelude::*,
    spi::{master::Spi, SpiMode},
    system::SystemControl,
};
use esp_println::println;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = SystemControl::new(peripherals.SYSTEM);

    let clocks = ClockControl::max(system.clock_control).freeze();
    let delay = Delay::new(&clocks);

    esp_println::logger::init_logger_from_env();

    log::info!("The program started");

    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);
    /// If you want to use different pins, feel free to change them here
    let sclk = io.pins.gpio18;
    let serial_in = io.pins.gpio19;
    let serial_out = io.pins.gpio23;
    let cs = Output::new(io.pins.gpio5, Level::Low);

    if cfg!(feature = "exclusive") {
        log::info!("Exclusive access to the SPI bus selected");

        let spi_driver = Spi::new(peripherals.SPI2, 1_000u32.kHz(), SpiMode::Mode0, &clocks)
            .with_sck(sclk)
            .with_mosi(serial_out)
            .with_miso(serial_in);

        let sd_spi_device = ExclusiveDevice::new(spi_driver, DummyCsPin, delay).unwrap();
        let sd_card = SdCard::new(sd_spi_device, cs, delay);

        println!("Size of the sd card: {:#?}", sd_card.num_bytes().unwrap());
    } else if cfg!(feature = "shared") {
        log::info!("Shared access to the SPI bus selected");

        let spi_driver = RefCell::new(
            Spi::new(peripherals.SPI2, 1_000u32.kHz(), SpiMode::Mode0, &clocks)
                .with_sck(sclk)
                .with_mosi(serial_out)
                .with_miso(serial_in),
        );

        let sd_spi_device = RefCellDevice::new(&spi_driver, DummyCsPin, delay).unwrap();
        let sd_card = SdCard::new(sd_spi_device, cs, delay);

        println!("Size of the sd card: {:#?}", sd_card.num_bytes().unwrap());
    }

    loop {
        log::info!("Loop..");
        delay.delay(500.millis());
    }
}
