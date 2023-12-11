#![no_std]
#![no_main]

extern crate cortex_m_rt as rt;
extern crate tudelft_lm3s6965_pac as _;
#[global_allocator]
static ALLOCATOR: emballoc::Allocator<1024> = emballoc::Allocator::new();

extern crate alloc;

use alloc::boxed::Box;
use crate::uart::Uart;
//use core::arch::asm;
use core::fmt::Write;
use cortex_m::asm;
use cortex_m_semihosting::{hprint, hprintln};
use drawing::brightness::Brightness;
use drawing::screen::Screen;
use rt::entry;
use tudelft_lm3s6965_pac::{Interrupt, NVIC, Peripherals};
use common_lib::Direction::{Up, Down, Left, Right};
use alloc::vec::Vec;
use serde::{Serialize, Deserialize};
use crc::{Crc, CRC_32_ISCSI};
use postcard::{from_bytes_cobs, from_bytes_crc32, to_allocvec_cobs, to_allocvec_crc32};
use common_lib::{deserialise, serialise};

use crate::mutex::Mutex;

mod drawing;
mod exceptions;
mod uart;
mod mutex;
mod ringbuffer;

static GLOBAL_UART: Mutex<Option<Uart>> = Mutex::new(None);

#[entry]
fn main() -> ! {
    // hprintln is kind of like cheating. On real hardware this is (usually)
    // not possible, but because we are running inside an emulator, we can
    // actually talk to the emulator and print to the stdout fo the emulator.
    // This is useful for debugging, but again: it doesn't work on real hardware.
    hprintln!("code started");
    let mut dp = Peripherals::take().unwrap();

    // initialize the screen for drawing
    let mut screen = Screen::new(&mut dp.SSI0, &mut dp.GPIO_PORTC);
    screen.clear(Brightness::WHITE);
    screen.define_starting_point(20, 50);
    // initialize the UART.
    dp.SYSCTL.dcgc1.write(|w|{
        w.sysctl_dcgc1_uart0().set_bit()
    });
    let uart = Uart::new(dp.UART0);

    GLOBAL_UART.update(|mut uart_inside| {
        *uart_inside = Some(uart);
    });
    unsafe { NVIC::unmask(Interrupt::UART0) };

    let a :u32 = 456765456;
    let serialised = serialise(a);

    // GLOBAL_UART.update(|u| {
    //     u.as_mut().unwrap().write(serialised.as_slice())
    // });
    GLOBAL_UART.update(|u| {
        u.as_mut().unwrap().write(serialised.as_slice())
    });

    // and write something to be received by the runner
    // GLOBAL_UART.update(|u| {
    //     writeln!(u.as_mut().unwrap(), "Hello, World!").unwrap();
    // });
    //writeln!(&mut uart, "Hello, World!").unwrap();

    // while true, see if we received new bytes (you should make it so
    // uart receives trigger an interrupt which push to some kind of
    // buffer so this read operation works)

    let mut rx_vec = Vec::new();
    let mut rx_data: u32 = 0;
    loop {

        for _ in 0..2000000 {asm::delay(1);}

            asm::delay(200);
            GLOBAL_UART.update(|u| {
                let byte = u.as_mut().unwrap().read();
                if u.as_mut().unwrap().uart.fr.read().uart_fr_rxfe().bit_is_set(){
                    hprint!("fifo empty");
                }
                if u.as_mut().unwrap().uart.fr.read().uart_fr_rxff().bit_is_set(){
                    hprint!("fifo full");
                }
                while byte != None {
                    //hprint!(" board fail 1");
                    let byte = byte.unwrap();
                    //hprint!("0x{:x}", byte);
                    rx_vec.push(byte);
                    if byte == 0x00{
                        let data = deserialise(rx_vec.as_mut_slice());
                        if data != None {
                            rx_data = data.unwrap();
                            rx_vec.clear();
                            break
                        }
                        else {
                            rx_vec.clear();
                            break;
                        }
                    }
                }
            });

            //asm::wfi();
            if rx_data == 456765456{
                hprint!(" board OK ");
            }
            else
            {
                hprint!(" board fail 0");
            }

        // wait for interrupts, before looping again to save cycles.
        // when the system is awake, run 10000 iterations before waiting for interrupts.
        // unsafe { asm!("wfi") }
        //asm::wfi();
    }
}



// pub fn deserialise(byte_slice: & mut [u8])  {
//     let crc = Crc::<u32>::new(&CRC_32_ISCSI);
//     // if let Ok(decoded) = from_bytes_cobs(byte_slice){
//     //     from_bytes_crc32(&decoded, crc.digest())
//     // }else {
//     //     Err(())
//     // }
//     let decoded: u32 = from_bytes_cobs(byte_slice).unwrap();
//     //let checked: u32 = from_bytes_crc32(&decoded, crc.digest()).unwrap();
//     hprint!("{:}", decoded);
// }