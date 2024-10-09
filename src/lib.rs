#![no_std]
use esp_backtrace as _;
use esp_hal::{clock::Clocks, peripherals::{RADIO_CLK, RNG, TIMG0}};

extern crate alloc;
use core::mem::MaybeUninit;

#[global_allocator]
pub static ALLOCATOR: esp_alloc::EspHeap = esp_alloc::EspHeap::empty();

pub fn init_heap() {
    const HEAP_SIZE: usize = 32 * 1024;
    static mut HEAP: MaybeUninit<[u8; HEAP_SIZE]> = MaybeUninit::uninit();

    unsafe {
        ALLOCATOR.init(HEAP.as_mut_ptr() as *mut u8, HEAP_SIZE);
    }
}

pub fn esp_init(timer: TIMG0, rng: RNG, radio: RADIO_CLK, clocks: Clocks<'_>) {
    let timg0 = esp_hal::timer::timg::TimerGroup::new(timer, &clocks);

    let _init = esp_wifi::initialize(
        esp_wifi::EspWifiInitFor::Wifi,
        timg0.timer0,
        esp_hal::rng::Rng::new(rng),
        radio,
        &clocks,
    )
    .unwrap();
}
