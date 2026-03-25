use embedded_hal::delay::DelayNs;
use rp235x_hal::{self as hal};

const XTAL_FREQ_HZ: u32 = 12_000_000u32;

use hal::{
    Clock,
    multicore::{Multicore, Stack},
    sio::Sio,
    timer::{CopyableTimer0, CopyableTimer1, Timer},
};
static CORE1_STACK: Stack<4096> = Stack::new();

pub struct PeriWrap {}
impl PeriWrap {
    fn get_pac() -> rp235x_hal::pac::Peripherals {
        unsafe { hal::pac::Peripherals::steal() }
    }
    pub fn init_core1() {
        let mut pac = PeriWrap::get_pac();
        let mut watchdog = hal::watchdog::Watchdog::new(pac.WATCHDOG);
        let mut sio = Sio::new(pac.SIO);
        let mut mc = Multicore::new(&mut pac.PSM, &mut pac.PPB, &mut sio.fifo);
        let cores = mc.cores();
        let core1 = &mut cores[1];
        let clocks = hal::clocks::init_clocks_and_plls(
            XTAL_FREQ_HZ,
            pac.XOSC,
            pac.CLOCKS,
            pac.PLL_SYS,
            pac.PLL_USB,
            &mut pac.RESETS,
            &mut watchdog,
        )
        .unwrap();
        let _test = core1.spawn(CORE1_STACK.take().unwrap(), move || {
            core1_main(clocks.system_clock.freq().to_Hz())
        });
    }
    pub fn get_pins() -> rp235x_hal::gpio::Pins {
        let mut pac = PeriWrap::get_pac();
        let sio = Sio::new(pac.SIO);
        let pins = hal::gpio::Pins::new(
            pac.IO_BANK0,
            pac.PADS_BANK0,
            sio.gpio_bank0,
            &mut pac.RESETS,
        );
        return pins;
    }
    pub fn get_sio() -> rp235x_hal::Sio {
        let pac = PeriWrap::get_pac();
        let sio = Sio::new(pac.SIO);
        return sio;
    }
    pub fn get_timer0() -> Timer<CopyableTimer0> {
        let mut pac = PeriWrap::get_pac();
        let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);
        let clocks = hal::clocks::init_clocks_and_plls(
            XTAL_FREQ_HZ,
            pac.XOSC,
            pac.CLOCKS,
            pac.PLL_SYS,
            pac.PLL_USB,
            &mut pac.RESETS,
            &mut watchdog,
        )
        .unwrap();
        Timer::new_timer0(pac.TIMER0, &mut pac.RESETS, &clocks)
    }
    pub fn get_timer1() -> Timer<CopyableTimer1> {
        let mut pac = PeriWrap::get_pac();
        let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);
        let clocks = hal::clocks::init_clocks_and_plls(
            XTAL_FREQ_HZ,
            pac.XOSC,
            pac.CLOCKS,
            pac.PLL_SYS,
            pac.PLL_USB,
            &mut pac.RESETS,
            &mut watchdog,
        )
        .unwrap();
        Timer::new_timer1(pac.TIMER1, &mut pac.RESETS, &clocks)
    }
}

use crate::data::{Data, ENV};
use crate::parser::Parser;
use crate::ports::ToString;
use crate::tokenizer::Tokenizer;

pub fn core1_main(sys_freq: u32) {
    let mut sio = PeriWrap::get_sio();
    let tru = Data::atom(&"#t".to_string());
    {
        let mut env = ENV.write();
        *env = Data::pair(tru.clone(), tru.clone(), Data::nil());
    };
    let mut timer = PeriWrap::get_timer1();
    for line in include_str!("../new.lisp").lines() {
        let env = { ENV.read().clone() };
        sio.fifo.write(2);
        sio.fifo.write(100);
        let _ = sio.fifo.read_blocking();
        timer.delay_ms(500);
        let mut p = Parser::new(Tokenizer::new(line));
        let code = p.eval();
        let _result = Data::eval(code, env.clone());
        sio.fifo.write(2);
        sio.fifo.write(100);
        let _ = sio.fifo.read_blocking();
    }
}
