#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m::asm;
use cortex_m_rt::entry;
use core::sync::atomic::AtomicBool;


const SIZE: usize = 100000;

#[cfg(not(feature = "trigger-overflow"))]
static NO_OVERFLOW: [u8; SIZE] = [42; SIZE];

#[cfg(feature = "trigger-overflow")]
static mut OVERFLOW: [u8; SIZE] = [42; SIZE];

#[entry]
fn main() -> ! {
    unsafe {
        // To avoid static removal
        let mut x = 0;

        #[cfg(not(feature = "trigger-overflow"))]
        for i in NO_OVERFLOW.as_ref().iter() {
            x += *i;
        }

        #[cfg(feature = "trigger-overflow")]
        for i in OVERFLOW.as_mut().iter_mut() {
            x += *i;
            *i = x;
        }

        asm::delay(x as u32);
        loop {}
    }
}
