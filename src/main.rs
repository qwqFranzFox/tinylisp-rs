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
use crate::ports::ToString;
use alloc::format;
use alloc::string::String;
use alloc_cortex_m::CortexMHeap as Heap;
use hal::entry;
use panic_halt as _;
use rp235x_hal::{self as hal};

use crate::data::{Data, ENV};
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
    #[allow(static_mut_refs)]
    let (mut res_write, res_read) = unsafe { RESULT.split() };
    #[allow(static_mut_refs)]
    let (next_write, mut next_read) = unsafe { NEXT_LINE.split() };

    PeriWrap::init_core1(next_write, res_read);

    {
        // run init and prelude
        let tru = Data::atom(&"#t".to_string());
        {
            let mut env = ENV.write();
            *env = Data::pair(tru.clone(), tru.clone(), Data::nil());
        };
        let parser = Parser::new(Tokenizer::new(include_str!("../prelude.lisp")));
        for code in parser {
            let env = { ENV.read().clone() };
            let _result = Data::eval(code, env);
        }
    }

    loop {
        if next_read.ready() {
            let block = next_read.dequeue().unwrap();
            let parser = Parser::new(Tokenizer::new(block.as_str()));
            for code in parser {
                let env = { ENV.read().clone() };
                let _ = res_write.enqueue(format!("Code: {code}"));
                let result = Data::eval(code, env);
                let _ = res_write.enqueue(format!("Result {result}"));
            }
        }
    }
}
