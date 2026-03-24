#![no_std]
#![no_main]
extern crate alloc;
mod data;
mod parser;
mod peri;
mod ports;
mod prims;
mod tokenizer;

use crate::peri::PeriWrap;
use alloc_cortex_m::CortexMHeap as Heap;
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;
use hal::entry;
use panic_halt as _;
use rp235x_hal as hal;

#[global_allocator]
static HEAP: Heap = Heap::empty();
const HEAP_SIZE: usize = 4 * 1024;
static mut HEAP_MEMORY: [core::mem::MaybeUninit<u8>; HEAP_SIZE] =
    [core::mem::MaybeUninit::uninit(); HEAP_SIZE];

fn heap_init() {
    unsafe {
        HEAP.init(&raw mut HEAP_MEMORY as usize, HEAP_SIZE);
    }
}

#[entry]
fn main() -> ! {
    heap_init();
    let mut led_pin = PeriWrap::get_pins().gpio25.into_push_pull_output();
    let mut sio = PeriWrap::get_sio();
    let mut timer = PeriWrap::get_timer0();
    let mut blink = move |n: u32, delay: u32| {
        for _ in 0..n {
            led_pin.set_high();
            timer.delay_ms(delay);
            led_pin.set_low();
            timer.delay_ms(delay);
        }
    };
    blink(1, 1000);
    PeriWrap::init_core1();
    loop {
        let num = sio.fifo.read_blocking();
        let delay = sio.fifo.read_blocking();
        blink(num, delay);
        sio.fifo.write(0);
    }
}
