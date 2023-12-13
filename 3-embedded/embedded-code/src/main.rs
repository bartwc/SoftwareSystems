#![no_std]
#![no_main]

extern crate cortex_m_rt as rt;
extern crate tudelft_lm3s6965_pac as _;

// Though each packet can be up to 9 bytes, we set the size of the heap
// to 1024 bytes to provide extra redundancy.
// The program can recover when the buffer fills up,
// but when the heap overflows, the program would crash.
// We must ensure that a heap overflow does not happen.
#[global_allocator]
static ALLOCATOR: emballoc::Allocator<1024> = emballoc::Allocator::new();

extern crate alloc;


use crate::uart::Uart;
//use core::arch::asm;


use cortex_m_semihosting::{hprintln};
use drawing::brightness::Brightness;
use drawing::screen::Screen;
use rt::entry;
use tudelft_lm3s6965_pac::{Interrupt, NVIC, Peripherals};

use alloc::vec::Vec;



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

    GLOBAL_UART.update(|uart_inside| {
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
                screen.reset_steps();
                if is_map_view {
                    screen.show_positions();
                }
                else {
                    screen.show_step_count();
                }
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

/*
The get_message function is designed to retrieve a DataFrame by reading
bytes from a UART interface and deserialising the received bytes.

It initializes a vector rx_vec to hold the received bytes and declares
a variable byte as an Option<u8>. It enters a loop that continues until
a byte is received.

Within this loop, it repeatedly updates the global UART interface using
the GLOBAL_UART.update method, attempting to read a byte into the byte variable.
Once a byte is received, it enters another loop to accumulate bytes until
a specific termination byte (0x00) is encountered. It uses the received
byte to populate rx_vec.

When the termination byte (0x00) is encountered, it attempts to deserialise
the accumulated bytes using the deserialise function. If deserialization
succeeds, it stores the deserialised DataFrame in return_val, clears rx_vec,
and breaks out of the loop to return the deserialised data.

If deserialisation fails, it clears rx_vec and continues to read more bytes.
The function returns the deserialised DataFrame.
 */
pub fn get_message() -> DataFrame {
    let mut rx_vec = Vec::new();
    //let mut rx_data: u32 = 0;
    let mut byte: Option<u8> = None;
    let return_val;

    while byte.is_none() {
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
            if data.is_some() {
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
        while byte.is_none() {
            GLOBAL_UART.update(|u| {
                byte = u.as_mut().unwrap().read();
            });
        }
    }
    return_val
}

/*
The send_message function takes a reference to a slice of bytes (&[u8])
as an argument and sends these bytes as a message using a global UART
(Universal Asynchronous Receiver-Transmitter) interface.

It uses a GLOBAL_UART variable, likely a global shared state, and calls
the update method on it. Within the closure passed to update, it uses the
as_mut method to obtain a mutable reference to the UART, and then calls
the write method on it, passing in the bytes slice, thus sending the bytes
over the UART interface.
 */
pub fn send_message(bytes: &[u8]){
    GLOBAL_UART.update(|u| {
        u.as_mut().unwrap().write(bytes)
    });
}