use core::fmt::Write;
use core::ops::Deref;
use cortex_m_semihosting::hprint;
use tudelft_lm3s6965_pac::{interrupt, Interrupt, NVIC};
use tudelft_lm3s6965_pac::UART0;
use crate::GLOBAL_UART;
use crate::ringbuffer::RingBuffer;

pub struct Uart {
    read_buffer: RingBuffer,
    write_buffer: RingBuffer,
    uart: UART0,
}

impl Uart {
    pub fn new(uart: UART0) -> Self {
        uart.ctl.write(|w| w.uart_ctl_uarten().clear_bit());
        uart.ibrd.write(|w| unsafe {
            w.uart_ibrd_divint().bits(10)
        });
        uart.fbrd.write(|w| unsafe {
            w.uart_fbrd_divfrac().bits(54)
        });
        uart.lcrh.write(|w| unsafe {
            w.bits(0x00000060)
        });
        uart.im.write(|w|{
            w.uart_im_rxim().set_bit()
        });
        // Unmask enables interrupt. It is unsafe because it may break critical sections.
        // It is sound because the initialisation of uart is not within a critical section.
        unsafe {NVIC::unmask(Interrupt::UART0)};

        uart.ctl.write(|w| w.uart_ctl_uarten().set_bit());

        Uart {
            read_buffer: RingBuffer::new(),
            write_buffer: RingBuffer::new(),
            uart
        }
    }

    pub fn write(&mut self, value: &[u8]) {
        for each in value {
            while self.uart.fr.read().uart_fr_txff().bit_is_set() {} //busy waiting while the fifo queue is full
            self.uart.dr.write(|w| unsafe {
                w.uart_dr_data().bits(*each)
            })
        }
    }

    pub fn read(&mut self) -> Option<u8> {
        // if self.uart.fr.read().uart_fr_rxfe().bit_is_clear() {
        //     Some(self.uart.dr.read().uart_dr_data().bits())
        // } else {
        //     None
        // }
        GLOBAL_UART.update(|uart|{
            if uart.as_mut().unwrap().read_buffer.num_bytes() == 0 {
                None
            } else {
                uart.as_mut().unwrap().read_buffer.pop_byte()
            }
        })
    }
}

impl Write for Uart {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write(s.as_bytes());
        Ok(())
    }
}

#[interrupt]
unsafe fn UART0() {
    GLOBAL_UART.update(|uart|{
        uart.as_mut().unwrap().uart.icr.write(|w|{
            w.uart_icr_rxic().set_bit()
        });
        if uart.as_mut().unwrap().read_buffer.space_remaining() > 0{
            let byte = uart.as_mut().unwrap().uart.dr.read().uart_dr_data().bits();
            uart.as_mut().unwrap().read_buffer.push_byte(byte);
        }
        else {
            hprint!("read buffer full");
        }
    });
    //hprint!("handler")
}
