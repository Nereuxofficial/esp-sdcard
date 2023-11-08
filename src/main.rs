//! This is an example of how to use an SD-Card with the ESP32 in no-std Rust.
#![no_std]
#![no_main]

extern crate alloc;

use alloc::vec;
use core::mem::MaybeUninit;
use embedded_sdmmc::{File, Mode, VolumeIdx};
use esp_backtrace as _;
use esp_println::println;
use hal::spi::master::Spi;
use hal::spi::SpiMode;
use hal::{clock::ClockControl, peripherals::Peripherals, prelude::*, Delay, IO};

#[global_allocator]
static ALLOCATOR: esp_alloc::EspHeap = esp_alloc::EspHeap::empty();

fn init_heap() {
    const HEAP_SIZE: usize = 32 * 1024;
    static mut HEAP: MaybeUninit<[u8; HEAP_SIZE]> = MaybeUninit::uninit();

    unsafe {
        ALLOCATOR.init(HEAP.as_mut_ptr() as *mut u8, HEAP_SIZE);
    }
}
struct FakeTimesource();

impl embedded_sdmmc::TimeSource for FakeTimesource {
    fn get_timestamp(&self) -> embedded_sdmmc::Timestamp {
        embedded_sdmmc::Timestamp {
            year_since_1970: 0,
            zero_indexed_month: 0,
            zero_indexed_day: 0,
            hours: 0,
            minutes: 0,
            seconds: 0,
        }
    }
}

struct FakeCs();

impl embedded_hal::digital::v2::OutputPin for FakeCs {
    type Error = core::convert::Infallible;
    fn set_low(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[entry]
fn main() -> ! {
    init_heap();
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();

    let clocks = ClockControl::max(system.clock_control).freeze();
    let mut delay = Delay::new(&clocks);

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let sclk = io.pins.gpio14;
    let miso = io.pins.gpio2;
    let mosi = io.pins.gpio15;
    let cs = io.pins.gpio13.into_push_pull_output();

    let mut spi = Spi::new_no_cs(
        peripherals.SPI2,
        sclk,
        mosi,
        miso,
        1000u32.kHz(),
        SpiMode::Mode0,
        &clocks,
    );
    println!("SPI initialized. Initializing SD-Card...");
    let sdcard = embedded_sdmmc::sdcard::SdCard::new(spi, cs, delay);

    println!("Card size is {} bytes", sdcard.num_bytes().unwrap());

    let mut volume_manager = embedded_sdmmc::VolumeManager::new(sdcard, FakeTimesource());

    let mut volume0 = volume_manager.open_volume(VolumeIdx(0)).unwrap();
    println!("Volume 0: {:?}", volume0);
    let root_dir = volume_manager.open_root_dir(volume0).unwrap();
    if let Ok(file) = volume_manager.open_file_in_dir(root_dir, "MY_FILE.TXT", Mode::ReadOnly) {
        println!("File opened: {:?}", file);
    }
    // setup logger
    // To change the log_level change the env section in .cargo/config.toml
    // or remove it and set ESP_LOGLEVEL manually before running cargo run
    // this requires a clean rebuild because of https://github.com/rust-lang/cargo/issues/10358
    esp_println::logger::init_logger_from_env();
    log::info!("Logger is setup");
    println!("Hello world!");
    loop {
        println!("Loop...");
        delay.delay_ms(500u32);
    }
}
