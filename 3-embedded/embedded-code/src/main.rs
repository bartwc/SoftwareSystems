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
use crate::mutex::Mutex;

mod drawing;
mod exceptions;
mod uart;
mod mutex;
mod ringbuffer;

// #[global_allocator]
// static HEAP: Heap = Heap::empty();

static GLOBAL_UART: Mutex<Option<Uart>> = Mutex::new(None);

#[entry]
fn main() -> ! {
    // {
    //     use core::mem::MaybeUninit;
    //     const HEAP_SIZE: usize = 1024;
    //     static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
    //     unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
    // }
    // hprintln is kind of like cheating. On real hardware this is (usually)
    // not possible, but because we are running inside an emulator, we can
    // actually talk to the emulator and print to the stdout fo the emulator.
    // This is useful for debugging, but again: it doesn't work on real hardware.
    hprintln!("code started");
    let mut dp = Peripherals::take().unwrap();

    // initialize the screen for drawing
    let mut screen = Screen::new(&mut dp.SSI0, &mut dp.GPIO_PORTC);
    screen.clear(Brightness::WHITE);

    // initialize the UART.
    let uart = Uart::new(dp.UART0);

    GLOBAL_UART.update(|mut uart_inside| {
        *uart_inside = Some(uart);
    });

    // and write something to be received by the runner
    GLOBAL_UART.update(|u| {
        writeln!(u.as_mut().unwrap(), "Hello, World!").unwrap();
    });
    //writeln!(&mut uart, "Hello, World!").unwrap();

    // while true, see if we received new bytes (you should make it so
    // uart receives trigger an interrupt which push to some kind of
    // buffer so this read operation works)
    loop {
        GLOBAL_UART.update(|u| {
            while let Some(i) = u.as_mut().unwrap().read() {
                hprint!("0x{:x}", i);
            }
        });
        // wait for interrupts, before looping again to save cycles.
        unsafe { asm!("wfi") }
    }
}
