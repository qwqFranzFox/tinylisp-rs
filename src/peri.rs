use alloc::format;
use alloc::string::ToString;
use embedded_hal::delay::DelayNs;
use heapless::spsc::Consumer;
use heapless::spsc::Producer;
use rp235x_hal::{self as hal};
use usb_device::class_prelude::*;
use usb_device::prelude::*;
use usbd_serial::SerialPort;

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
    pub fn init_core1(
        next_write: Producer<'static, String>,
        result_read: Consumer<'static, String>,
    ) {
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
            core1_main(clocks.system_clock.freq().to_Hz(), next_write, result_read);
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
    pub fn get_usb() -> usb_device::bus::UsbBusAllocator<rp235x_hal::usb::UsbBus> {
        // from rp235x_hal example repo
        // Set up the USB driver
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
        let usb_bus = UsbBusAllocator::new(hal::usb::UsbBus::new(
            pac.USB,
            pac.USB_DPRAM,
            clocks.usb_clock,
            true,
            &mut pac.RESETS,
        ));
        return usb_bus;
    }
    pub fn get_serial_from_usb(
        usb_bus: &usb_device::bus::UsbBusAllocator<rp235x_hal::usb::UsbBus>,
    ) -> (
        SerialPort<'_, rp235x_hal::usb::UsbBus>,
        usb_device::device::UsbDevice<'_, rp235x_hal::usb::UsbBus>,
    ) {
        // Set up the USB Communications Class Device driver
        let serial = SerialPort::new(&usb_bus);

        // Create a USB device with a fake VID and PID
        let usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
            .strings(&[StringDescriptors::default()
                .manufacturer("qwqFranzFox")
                .product("Serial port")
                .serial_number("TEST")])
            .unwrap()
            .max_packet_size_0(USB_SERIAL_BUF_SIZE as u8)
            .unwrap()
            .device_class(2) // from: https://www.usb.org/defined-class-codes
            .build();
        return (serial, usb_dev);
    }
}

use crate::ports::String;
const USB_SERIAL_BUF_SIZE: usize = 64;

struct StateMachine {
    buffer: String,
}

impl StateMachine {
    pub fn new() -> StateMachine {
        StateMachine {
            buffer: String::new(),
        }
    }

    pub fn next(&mut self, c: char) -> (Option<String>, Option<String>) {
        match c {
            '\x08' | '\x7f' => (
                None,
                match self.buffer.pop() {
                    Some(_) => Some("\x08 \x08".to_string()),
                    None => None,
                },
            ),
            '\n' | '\r' => {
                let k = self.buffer.clone();
                self.buffer.clear();
                return (Some(k), Some("\r\n".to_string()));
            }
            c => {
                self.buffer.push(c);
                (None, Some(c.to_string()))
            }
        }
    }
}

pub fn core1_main(
    _sys_freq: u32,
    mut next_write: Producer<String>,
    mut result_read: Consumer<String>,
) {
    let usb_bus = PeriWrap::get_usb();
    let (mut serial, mut usb_dev) = PeriWrap::get_serial_from_usb(&usb_bus);
    let mut timer = PeriWrap::get_timer1();
    let mut newline = true;
    let mut machine: StateMachine = StateMachine::new();
    loop {
        if newline {
            if let Ok(_) = serial.write(b">>> ") {
                newline = false;
            }
        }

        if usb_dev.poll(&mut [&mut serial]) {
            let mut buf: [u8; 1] = [0];
            match serial.read(&mut buf) {
                Ok(0) => {}
                Err(_err) => {}
                Ok(_size) => {
                    let buf = buf[0] as char;
                    let (line, reply) = machine.next(buf);
                    if let Some(reply) = reply {
                        let _ = serial.write(reply.as_bytes());
                    }
                    if let Some(line) = line {
                        let _ = next_write.enqueue(line);
                        // TODO: idk why a delay of 100ms could solve deadlock... but it does
                        timer.delay_ms(100);
                    }
                }
            }
        }
        if result_read.ready() {
            let result = result_read.dequeue().unwrap();
            let _ = serial.write(format!("Result:\r\n{result}\r\n").as_bytes());
            newline = true;
        }
    }
}
