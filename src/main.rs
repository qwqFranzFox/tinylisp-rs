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
use crate::ports::Vec;
use alloc::format;
use alloc::string::String;
use alloc_cortex_m::CortexMHeap as Heap;
use embedded_hal::digital::OutputPin;
use hal::entry;
use panic_halt as _;
use rp235x_hal::{self as hal};

use crate::data::Lisp;
use crate::parser::Parser;
use crate::tokenizer::Tokenizer;

#[global_allocator]
static HEAP: Heap = Heap::empty();
const HEAP_SIZE: usize = 16 * 1024;
static mut HEAP_MEMORY: [core::mem::MaybeUninit<u8>; HEAP_SIZE] =
    [core::mem::MaybeUninit::uninit(); HEAP_SIZE];

fn heap_init() {
    unsafe {
        HEAP.init(&raw mut HEAP_MEMORY as usize, HEAP_SIZE);
    }
}
use heapless::spsc::Queue;
static mut NEXT_LINE: Queue<String, 64> = Queue::new();
static mut RESULT: Queue<String, 64> = Queue::new();
#[entry]
fn main() -> ! {
    heap_init();

    let mut _sio = PeriWrap::get_sio();
    let mut _timer = PeriWrap::get_timer0();
    let mut a = PeriWrap::get_pins().gpio25.into_push_pull_output();
    #[allow(static_mut_refs)]
    let (mut res_write, res_read) = unsafe { RESULT.split() };
    #[allow(static_mut_refs)]
    let (next_write, mut next_read) = unsafe { NEXT_LINE.split() };

    PeriWrap::init_core1(next_write, res_read);
    let mut lisp = Lisp::new();

    {
        // run init and prelude
        let parser = Parser::new(Tokenizer::new(&include_str!("../prelude.lisp")));
        // a.set_high();
        let iter = parser.chain_eval(&mut lisp);
        let _results: Vec<_> = iter.collect();
        // BUG:: Writing to serial before connection cause crash
        // for result in results {
        //     let _ = res_write.enqueue(format!(
        //         ">>> Result: {}",
        //         result.debug(lisp.get_context_mut())
        //     ));
        // }
        lisp.get_context_mut().garbage_collection();
    }

    loop {
        if next_read.ready() {
            a.set_high();
            let block = next_read.dequeue().unwrap();
            let mut parser = Parser::new(Tokenizer::new(&block));
            let code = parser.eval(lisp.get_context_mut());
            // a.set_low();
            let result = lisp.eval(code);
            let _ = res_write.enqueue(format!("Result {}", result.debug(lisp.get_context_mut())));
            // let results: alloc::vec::Vec<_> = iter.collect();
            // for result in results {
            //     let _ =
            //         res_write.enqueue(format!("Result {}", result.debug(lisp.get_context_mut())));
            // }
            lisp.get_context_mut().garbage_collection();
        }
    }
}
