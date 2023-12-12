#![no_std]
#![no_main]

extern crate cortex_m_rt as rt;
extern crate tudelft_lm3s6965_pac as _;

#[global_allocator]
static ALLOCATOR: emballoc::Allocator<2048> = emballoc::Allocator::new();

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
use common_lib::{DataFrame, deserialise, PayLoad, serialise};

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
    screen.show_positions();
    // initialize the UART.
    dp.SYSCTL.dcgc1.write(|w| {
        w.sysctl_dcgc1_uart0().set_bit()
    });
    let uart = Uart::new(dp.UART0);

    GLOBAL_UART.update(|mut uart_inside| {
        *uart_inside = Some(uart);
    });

    /*
        It is unsafe when unmask enables interrupt because
        it may break masked-based critical sections.
        It is sound because the initialisation of uart is
        not within a critical section.
        SAFETY: According to https://docs.rs/cortex-m/0.6.7/cortex_m/peripheral/struct.NVIC.html#method.unmask
        unsafe feature is not accessed as utilisation of
        unmask enables interrupt is not within masked-based critical section
        */
    unsafe { NVIC::unmask(Interrupt::UART0) };


    // GLOBAL_UART.update(|u| {
    //     u.as_mut().unwrap().write(serialised.as_slice())
    // });
    // GLOBAL_UART.update(|u| {
    //     u.as_mut().unwrap().write(serialised.as_slice())
    // });

    // and write something to be received by the runner
    // GLOBAL_UART.update(|u| {
    //     writeln!(u.as_mut().unwrap(), "Hello, World!").unwrap();
    // });
    //writeln!(&mut uart, "Hello, World!").unwrap();

    // while true, see if we received new bytes (you should make it so
    // uart receives trigger an interrupt which push to some kind of
    // buffer so this read operation works)

    // let mut rx_vec = Vec::new();
    // let mut rx_data: u32 = 0;
    let mut is_map_view = true;
    loop {
        let msg = get_message();

        match msg.payload {
            PayLoad::TakeStep(direction) => {
                screen.take_step(direction);
                if is_map_view {
                    screen.show_positions();
                }
                else {
                    screen.show_step_count();
                }
            }
            PayLoad::ChangeView => {
                if is_map_view {
                    screen.show_step_count();
                    is_map_view = false;
                }
                else {
                    screen.show_positions();
                    is_map_view = true;
                }
            }
            PayLoad::Clear => {
                is_map_view = true;
                screen.reset_steps();
            }

            PayLoad::StepCountRequest => {
                let step_count = screen.get_step_count();
                let message = DataFrame{
                    payload: PayLoad::StepCount(step_count),
                };
                send_message(serialise(message).as_slice());
            }
            PayLoad::StepCount(_) => {}
        }
    }
}


pub fn get_message() -> DataFrame {
    let mut rx_vec = Vec::new();
    //let mut rx_data: u32 = 0;
    let mut byte: Option<u8> = None;
    let mut return_val;

    while byte == None {
        GLOBAL_UART.update(|u| {
            byte = u.as_mut().unwrap().read();
        });
    }
    // if u.as_mut().unwrap().uart.fr.read().uart_fr_rxfe().bit_is_set(){
    //     hprint!("fifo empty");
    // }
    // if u.as_mut().unwrap().uart.fr.read().uart_fr_rxff().bit_is_set(){
    //     hprint!("fifo full");
    // }
    loop {
        //hprint!(" board fail 1");
        let byte_inner = byte.unwrap();
        //hprint!("0x{:x}", byte_inner);
        rx_vec.push(byte_inner);
        if byte_inner == 0x00 {
            let data = deserialise(rx_vec.as_mut_slice());
            if data != None {
                return_val = data.unwrap();
                rx_vec.clear();
                break;
            } else {
                //hprint!("message failed");
                rx_vec.clear();
                //break;
            }
        }
        byte = None;
        while byte == None {
            GLOBAL_UART.update(|u| {
                byte = u.as_mut().unwrap().read();
            });
        }
    }
    return_val
}

pub fn send_message(bytes: &[u8]){
    GLOBAL_UART.update(|u| {
        u.as_mut().unwrap().write(bytes)
    });
}