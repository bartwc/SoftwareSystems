use core::fmt::Write;
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
            // It is unsafe because it writes raw bits to a register field, and there is no guarantee that the field is safe to write to.
            // It is sound because the uart_ibrd_divint field is safe to write to when we set the baud rate and initialise the uart driver.
            w.uart_ibrd_divint().bits(10)
        });
        uart.fbrd.write(|w| unsafe {
            // It is unsafe because it writes raw bits to a register field, and there is no guarantee that the field is safe to write to.
            // It is sound because the uart_fbrd_divfrac field is safe to write to when we set the baud rate and initialise the uart driver.
            w.uart_fbrd_divfrac().bits(54)
        });
        uart.lcrh.write(|w| unsafe {
            // It is unsafe because it writes raw bits to a register field, and there is no guarantee that the field is safe to write to.
            // It is sound because the lcrh register is safe to write to when we initialise the uart driver for the first time.
            w.bits(0x00000060)
        });
        uart.im.write(|w| {
            w.uart_im_rxim().set_bit()
        });
        // uart.im.write(|w|{
        //     w.uart_im_txim().set_bit()
        // });
        NVIC::unpend(Interrupt::UART0);
        // Unmask enables interrupt. It is unsafe because it may break critical sections.
        // It is sound because the initialisation of uart is not within a critical section.
        unsafe { NVIC::unmask(Interrupt::UART0) };

        uart.ctl.write(|w| w.uart_ctl_uarten().set_bit());

        Uart {
            read_buffer: RingBuffer::new(),
            write_buffer: RingBuffer::new(),
            uart,
        }
    }

    pub fn write(&mut self, value: &[u8]) { // Required for Protocol - Embedded Push Array (of Bytes) into Buffer
        for byte in value{
            if self.write_buffer.space_remaining() >= 1 {
                let write_result = self.write_buffer.push_byte(*byte);
                if write_result == Err(()) {
                    hprint!("write buffer full");
                }
            } else {
                self.write_to_uart();
                let write_result = self.write_buffer.push_byte(*byte);
                if write_result == Err(()) {
                    hprint!("write buffer full");
                }
            }
        }
        self.write_to_uart(); // Flush Buffer into PC
    }


    pub fn write_byte(&mut self, value: u8) { // Required for Protocol - Embedded Push Byte into Buffer
        if self.write_buffer.space_remaining() >= 1 {
            let write_result = self.write_buffer.push_byte(value);
            if write_result == Err(()) {
                hprint!("write buffer full");
            }
        } else {
            self.write_to_uart();
            let write_result = self.write_buffer.push_byte(value);
            if write_result == Err(()) {
                hprint!("write buffer full");
            }
        }
        self.write_to_uart(); // Flush Buffer into PC
    }

    fn write_to_uart(&mut self) {
        while self.write_buffer.num_bytes() >= 1 {
            if self.uart.fr.read().uart_fr_txff().bit_is_clear() {
                let byte = self.write_buffer.pop_byte().unwrap();
                self.uart.dr.write(|w| unsafe {
                    // It is unsafe because it writes raw bits to a register field, and there is no guarantee that the field is safe to write to.
                    // It is sound because we have checked in line 89 that the uart tx fifo queue or register is not full, and we can safely
                    // write the data into the register for transmission.
                    w.uart_dr_data().bits(byte)
                })
            }
        }
    }

    pub fn read(&mut self) -> Option<u8> { // Required for Protocol - Retrieves Info From PC with Intermediate Buffer
        if self.read_buffer.num_bytes() == 0 {
            None
        } else {
            self.read_buffer.pop_byte()
        }
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
    GLOBAL_UART.update(|uart| {
        if uart.as_mut().unwrap().uart.mis.read().uart_mis_rxmis().bit_is_set() {
            //hprint!("handler rx");
            uart.as_mut().unwrap().uart.icr.write(|w| {
                w.uart_icr_rxic().set_bit()
            });
            if uart.as_mut().unwrap().read_buffer.space_remaining() > 0 {
                let byte = uart.as_mut().unwrap().uart.dr.read().uart_dr_data().bits();
                let result_push = uart.as_mut().unwrap().read_buffer.push_byte(byte);
                if result_push == Err(()) {
                    hprint!("read buffer full");
                }
            } else {
                hprint!("read buffer full");
            }
        }
    });
    //hprint!("handler")
}
