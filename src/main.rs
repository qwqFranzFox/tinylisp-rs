#![no_std]
#![no_main]
extern crate alloc;
mod parser;
mod ports;
mod prims;
mod tokenizer;
mod types;

use crate::ports::ToString;
use crate::{
    parser::Parser,
    tokenizer::Tokenizer,
    types::{Data, ENV},
};
use alloc_cortex_m::CortexMHeap as Heap;
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;
use panic_halt as _;
use rp235x_hal::entry;
use rp235x_hal::{self as hal, Timer};

#[global_allocator]
static HEAP: Heap = Heap::empty();
const HEAP_SIZE: usize = 4096;
static mut HEAP_MEMORY: [core::mem::MaybeUninit<u8>; HEAP_SIZE] =
    [core::mem::MaybeUninit::uninit(); HEAP_SIZE];

fn heap_init() {
    unsafe {
        HEAP.init(&raw mut HEAP_MEMORY as usize, HEAP_SIZE);
    }
}

fn peri_init() -> (
    Timer<rp235x_hal::timer::CopyableTimer0>,
    rp235x_hal::gpio::Pins,
) {
    const XOSC_FREQ_HZ: u32 = 12_000_000u32; // 12MHz
    let mut peri = hal::pac::Peripherals::take().unwrap();
    let mut watchdog = hal::watchdog::Watchdog::new(peri.WATCHDOG);
    let clock = hal::clocks::init_clocks_and_plls(
        XOSC_FREQ_HZ,
        peri.XOSC,
        peri.CLOCKS,
        peri.PLL_SYS,
        peri.PLL_USB,
        &mut peri.RESETS,
        &mut watchdog,
    )
    .unwrap();
    let timer = hal::Timer::new_timer0(peri.TIMER0, &mut peri.RESETS, &clock);

    let sio = hal::Sio::new(peri.SIO);

    let pins = hal::gpio::Pins::new(
        peri.IO_BANK0,
        peri.PADS_BANK0,
        sio.gpio_bank0,
        &mut peri.RESETS,
    );
    (timer, pins)
}

#[entry]
fn main() -> ! {
    let (mut timer, pins) = peri_init();
    let mut led_pin = pins.gpio25.into_push_pull_output();
    const DELAY: u32 = 200;
    heap_init();
    let mut blink = |n: usize, timer: &mut Timer<_>| {
        for _ in 0..n {
            led_pin.set_high();
            timer.delay_ms(DELAY);
            led_pin.set_low();
            timer.delay_ms(DELAY);
        }
    };

    let tru = Data::atom(&"#t".to_string());
    {
        let mut env = ENV.write();
        *env = Data::pair(tru.clone(), tru.clone(), Data::nil());
    };
    for line in include_str!("../new.lisp").lines() {
        let env = { ENV.read().clone() };
        let mut p = Parser::new(Tokenizer::new(line));
        let code = p.eval();
        // info!("running: {}", code);
        blink(10, &mut timer);
        timer.delay_ms(1000);
        let result = Data::eval(code, env.clone());
        if let Data::Number(num) = *result {
            blink(num as usize, &mut timer);
        }
        timer.delay_ms(1000);
        // info!("{}", result);
        // info!("Env is : {}", env);
    }
    loop {}
}
