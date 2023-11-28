#![no_std]
#![no_main]

extern crate cortex_m_rt as rt;
extern crate tudelft_lm3s6965_pac as _;

use crate::uart::Uart;
use core::arch::asm;
use core::fmt::Write;
use cortex_m_semihosting::{hprint, hprintln};
use drawing::brightness::Brightness;
use drawing::font::ZERO;
use drawing::screen::Screen;
use rt::entry;
use tudelft_lm3s6965_pac::Peripherals;

mod drawing;
mod exceptions;
mod uart;

mod mutex;

#[entry] // start
fn main() -> ! { // returns something if there is no output in the loop
    // hprintln is kind of like cheating. On real hardware this is (usually)
    // not possible, but because we are running inside an emulator, we can
    // actually talk to the emulator and print to the stdout fo the emulator.
    // This is useful for debugging, but again: it doesn't work on real hardware.
    hprintln!("code started"); // semi-hosting (communicate to the outside world)
    let mut dp = Peripherals::take().unwrap(); // take the peripherals - unwrap peripherals to ensure mutual exclusive access - all peripherals are address

    // initialize the screen for drawing
    let mut screen = Screen::new(&mut dp.SSI0, &mut dp.GPIO_PORTC); // SSI is a protocol device - Abstraction Level - Page 488 of LM3S6965 Datasheet
    screen.clear(Brightness::WHITE); // clear with WHITE screen

    // initialize the UART.
    let mut uart = Uart::new(dp.UART0); // todo!

    // and write something to be received by the runner
    writeln!(&mut uart, "Hello, World!").unwrap();

    // while true, see if we received new bytes (you should make it so
    // uart receives trigger an interrupt which push to some kind of
    // buffer so this read operation works)
    loop {
        while let Some(i) = uart.read() {
            hprint!("0x{:x}", i);
        }

        // wait for interrupts, before looping again to save cycles.
        unsafe { asm!("wfi") }
    }
}

//check
