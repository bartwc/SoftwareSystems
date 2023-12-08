use core::fmt::Write;
use tudelft_lm3s6965_pac::interrupt;
use tudelft_lm3s6965_pac::UART0;
use crate::mutex::Mutex;

pub struct Uart {
    // read_buffer: ConstGenericRingBuffer<u8, 256>,
    // write_buffer: ConstGenericRingBuffer<u8, 256>,
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
        // uart.lcrh.write(|w|{
        //     w.uart_lcrh_fen().clear_bit()
        // });
        uart.lcrh.write(|w| unsafe {
            w.bits(0x00000060)
        });
        uart.ctl.write(|w| w.uart_ctl_uarten().set_bit());

        Uart { uart }
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
        if self.uart.fr.read().uart_fr_rxfe().bit_is_clear() {
            Some(self.uart.dr.read().uart_dr_data().bits())
        } else {
            None
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
unsafe fn UART0() {}
