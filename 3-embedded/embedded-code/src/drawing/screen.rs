use crate::drawing::brightness::Brightness;
use crate::drawing::font::NUMBERS;
use tudelft_lm3s6965_pac::{GPIO_PORTC, SSI0};
use common_lib::Direction;

pub struct Screen<'p> {
    ssi: &'p mut SSI0,
    gpio: &'p mut GPIO_PORTC,
    fb: [[u8; (Screen::WIDTH / 2) as usize]; Screen::HEIGHT as usize],
    step_count: usize,
    dy: i16,
    dx: i16,
    origin_y: u8,
    origin_x: u8,
    map: [[bool; Screen::WIDTH as usize]; Screen::HEIGHT as usize],
}

impl<'p> Screen<'p> {
    pub const WIDTH: u8 = 128;
    pub const HEIGHT: u8 = 80;

    pub fn new(ssi: &'p mut SSI0, gpio: &'p mut GPIO_PORTC) -> Self {
        // 1. Ensure that the SSE bit in the SSICR1 register is disabled before making any configuration changes.
        ssi.cr1.write(|w| w.ssi_cr1_sse().clear_bit());

        // 2. Select whether the SSI is a master or slave:
        //     a. For master operations, set the SSICR1 register to 0x0000.0000.
        //     b. For slave mode (output enabled), set the SSICR1 register to 0x0000.0004.
        //     c. For slave mode (output disabled), set the SSICR1 register to 0x0000.000C.
        ssi.cr1.write(|w| w.ssi_cr1_ms().clear_bit());

        // 3. Configure the clock prescale divisor by writing the SSICPSR register.
        // SAFETY: according to the docs, 2 is a valid value for this register
        ssi.cpsr.write(|w| unsafe { w.ssi_cpsr_cpsdvsr().bits(2) });

        // 4. Write the SSICR0 register with the following configuration:
        //     ■ Serial clock rate (SCR)
        //     ■ Desired clock phase/polarity, if using Freescale SPI mode (SPH and SPO)
        //     ■ The protocol mode: Freescale SPI, TI SSF, MICROWIRE (FRF)
        //     ■ The data size (DSS)
        // SAFETY: according to the docs, 9 is a valid value for this register
        ssi.cr0.write(|w| unsafe { w.ssi_cr0_scr().bits(9) });

        // 5. Enable the SSI by setting the SSE bit in the SSICR1 register.
        ssi.cr1.write(|w| w.ssi_cr1_sse().set_bit());

        // 6. set the bitmask
        ssi.cr0.write(|w| w.ssi_cr0_dss().ssi_cr0_dss_16());

        // SAFETY: according to the docs, these are both valid values for these two registers
        gpio.den.write(|w| unsafe { w.bits(1) });
        gpio.dir.write(|w| unsafe { w.bits(0xff) });

        Self {
            ssi,
            gpio,
            fb: [[0; (Self::WIDTH / 2) as usize]; Self::HEIGHT as usize],
            step_count: 0,
            dy: 0,
            dx: 0,
            origin_y: 0,
            origin_x: 0,
            map: [[false; Screen::WIDTH as usize]; Screen::HEIGHT as usize],
        }
    }

    fn write_ssi(&mut self, data: u16) {
        self.ssi.dr.write(|w| unsafe { w.ssi_dr_data().bits(data) });
        let _ = self.ssi.dr.read();
        while self.ssi.sr.read().ssi_sr_bsy().bit_is_set() {}
    }

    fn change_mode(&mut self, mode: Mode) {
        // SAFETY: these two values registers can have any 7 bit value.
        // the exact values correspond with the ones qemu expects here
        // which we checked by reading qemu's source code.
        match mode {
            Mode::Cmd => self.gpio.data.write(|w| unsafe { w.bits(0x00) }),
            Mode::Data => self.gpio.data.write(|w| unsafe { w.bits(0xa0) }),
        }
    }

    /// Assumes we are in command mode and min/max are in bounds
    fn set_col(&mut self, min_curr: u8, max: u8) {
        self.write_ssi(0x15);
        self.write_ssi(min_curr as u16);
        self.write_ssi(max as u16);
    }

    /// Assumes we are in command mode and min/max are in bounds
    fn set_row(&mut self, min_curr: u8, max: u8) {
        self.write_ssi(0x75);
        self.write_ssi(min_curr as u16);
        self.write_ssi(max as u16);
    }

    pub fn draw_pixel(&mut self, x: u8, y: u8, brightness: Brightness) {
        assert!(x < Screen::WIDTH, "x larger than width");
        assert!(y < Screen::HEIGHT, "y larger than height");

        self.change_mode(Mode::Cmd);
        self.set_col(x / 2, Self::WIDTH - 1);
        self.set_row(y, Self::HEIGHT - 1);

        self.change_mode(Mode::Data);

        let current = &mut self.fb[y as usize][x as usize / 2];
        if x % 2 == 1 {
            *current &= 0xf0;
            *current |= Into::<u8>::into(brightness);
        } else {
            *current &= 0x0f;
            *current |= Into::<u8>::into(brightness) << 4;
        }

        let value = *current;
        self.write_ssi(value as u16);
    }

    pub fn draw_unsigned_int(&mut self, x: u8, y: u8, brightness: Brightness, number: usize) {
        let mut number_left = number;
        let mut single_digit: u8;
        let mut x= x;

        if number_left == 0 {
            self.draw_digit(x, y, brightness, 0);
        }
        while number_left > 0 {
            single_digit = (number_left % 10) as u8;
            self.draw_digit(x, y, brightness, single_digit);
            number_left = number_left / 10;
            x = x - 9;
        }
    }

    fn draw_digit(&mut self, x: u8, y: u8, brightness: Brightness, digit: u8) {
        let mut x_i: u8 = 0;
        let mut y_i: u8 = 0;
        while y_i <= 15 {
            x_i = 0;
            while x_i <= 7  {
                if NUMBERS[digit as usize][y_i as usize][x_i as usize] {
                    self.draw_pixel(x + x_i, y + y_i, brightness);
                }
                x_i = x_i + 1;
            }
            y_i = y_i + 1;
        }
    }

    pub fn define_starting_point(&mut self, y: u8, x: u8){
        self.origin_x = x;
        self.origin_y = y;
        self.map[y as usize][x as usize] = true;
    }

    pub fn take_step(&mut self, direction: Direction) {
        match direction {
            Direction::Left => if self.origin_x as i16 + self.dx >= 0 { self.dx =  self.dx - 1},
            Direction::Right => if self.origin_x as i16 + self.dx <= Screen::WIDTH as i16 { self.dx =  self.dx + 1},
            Direction::Down => if self.origin_y as i16 + self.dy <= Screen::HEIGHT as i16 { self.dy =  self.dy + 1},
            Direction::Up => if self.origin_y as i16 + self.dy >= 0 { self.dy =  self.dy - 1},
        }
        let x = self.origin_x as i16 + self.dx;
        let y = self.origin_y as i16 + self.dy;
        self.step_count = self.step_count + 1;
        self.map[y as usize][x as usize] = true;
    }

    pub fn show_positions(&mut self) {
        self.clear(Brightness::WHITE);
        let mut x: u8 = 0;
        let mut y: u8 = 0;
        while y <= Screen::HEIGHT - 1 {
            x = 0;
            while x <= Screen::WIDTH - 1 {
                if self.map[y as usize][x as usize] {
                    self.draw_pixel(x, y, Brightness::BLACK);
                }
                x = x + 1;
            }
            y = y + 1;
        }
    }

    pub fn show_step_count(&mut self) {
        self.clear(Brightness::WHITE);
        self.draw_unsigned_int(119, 1, Brightness::BLACK, self.step_count);
    }

    pub fn reset_steps(&mut self) {
        self.step_count = 0;
        self.dy = 0;
        self.dx = 0;
        self.map = [[false; Screen::WIDTH as usize]; Screen::HEIGHT as usize];
        self.map[self.origin_y as usize][self.origin_x as usize] = true;
        self.show_positions()
    }

    pub fn clear(&mut self, brightness: Brightness) {
        self.change_mode(Mode::Cmd);
        self.set_row(0, Self::HEIGHT - 1);
        self.set_col(0, Self::WIDTH - 1);

        self.change_mode(Mode::Data);

        let brightness: u8 = brightness.into();
        let pix = brightness | (brightness << 4);

        for x in 0..(Self::WIDTH / 2) as usize {
            for y in 0..Self::HEIGHT as usize {
                self.write_ssi(pix as u16);
                self.fb[y][x] = pix;
            }
        }
    }
}

enum Mode {
    Cmd,
    Data,
}
