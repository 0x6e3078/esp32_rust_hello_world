#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl, delay::Delay, peripherals::{Peripherals, RADIO_CLK}, prelude::*, system::SystemControl,
};
use hello_world::{esp_init, init_heap};

#[entry]
fn main() -> ! {
    let peripherals: Peripherals = Peripherals::take();
    let system = SystemControl::new(peripherals.SYSTEM);
    let clocks = ClockControl::max(system.clock_control).freeze();
    let delay = Delay::new(&clocks);
    init_heap();

    let timer = peripherals.TIMG0;
    let rng = peripherals.RNG;
    let radio: RADIO_CLK = peripherals.RADIO_CLK;

    esp_init(timer, rng, radio, clocks);

    esp_println::logger::init_logger_from_env();

    loop {
        log::info!("Hello world!");
        delay.delay(500.millis());
    }
}
