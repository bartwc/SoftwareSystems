use crate::drawing::brightness::Brightness;
use crate::drawing::font::NUMBERS;
use tudelft_lm3s6965_pac::{GPIO_PORTC, SSI0};
use common_lib::Direction;

pub struct Screen<'p> {
    ssi: &'p mut SSI0,
    gpio: &'p mut GPIO_PORTC,
    fb: [[u8; (Screen::WIDTH / 2) as usize]; Screen::HEIGHT as usize],
    // Added fields below for Screen struct
    step_count: u32,
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
            // Added initial values for newly created fields in Screen struct
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


    /*
    The function draw_unsigned_int takes in &mut self as an argument, indicating that
    this function mutates the instance of the object it's called on. A mutable variable
    number_left which is initially set to number.

    It first checks if number_left is equal to 0. If yes, it calls the draw_digit method
    on self with x coordinates, y coordinates, brightness and value 0 to be drawn.

    If number is not 0, it enters a while loop that continues until number_left is >0.
    This loop breaks down the number into individual digits and draws each one from
    left to right.

    First it calculates the remainder of number_left divided by 10, casts the result
    from u32 to u8 to match the type of single_digit. Then it calls draw_digit which
    draws the digit at the given coordinates with the specified brightness level.

    Finally it moves left by 10 pixels, making the space between single digits to be
    2 pixels.
     */
    pub fn draw_unsigned_int(&mut self, x: u8, y: u8, brightness: Brightness, number: u32) {
        let mut number_left = number;
        let mut single_digit: u8;
        let mut x= x;

        if number_left == 0 {
            self.draw_digit(x, y, brightness, 0);
        }
        while number_left > 0 {
            single_digit = (number_left % 10) as u8;
            self.draw_digit(x, y, brightness, single_digit);
            number_left /= 10;
            x -= 10;
        }
    }

    /*
    The function draw_digit takes in &mut self as an argument, indicating that
    this function mutates the instance of the object it's called on. (x, y)
    are coordinates where the digit is to be drawn. The other two parameters
    brightness and digit refers to the value that will be drawn with its
    respective brightness value.

    Within the outer loop y_i, it will run 16 times and the inner loop x_i will
    run 8 times to scan through the 2D boolean array dimensioned 8 x 16 and only
    draw the pixels that are labelled true. All 8 column y_i values will be
    scanned for the first row, followed by repeating the process for the next
    15 rows.
     */
    fn draw_digit(&mut self, x: u8, y: u8, brightness: Brightness, digit: u8) {
        let mut x_i: u8;
        let mut y_i: u8 = 0;
        while y_i <= 15 {
            x_i = 0;
            while x_i <= 7  {
                if NUMBERS[digit as usize][y_i as usize][x_i as usize] {
                    self.draw_pixel(x + x_i, y + y_i, brightness);
                }
                x_i += 1;
            }
            y_i += 1;
        }
    }

    /*
    The function define_starting point takes in &mut self as an argument, indicating
    that this function mutates the instance of the object it's called on and sets
    the origin of the black pixel in the 128 x 80 px screen. The origin of the black
    pixel is defined by data variables origin_x and origin_y shown on the 2D screen.

    y as usize and x as usize are explicit type conversions: they convert y and
    x from u8 to usize, which is necessary because array indices in Rust are of
    type usize.
     */
    pub fn define_starting_point(&mut self, y: u8, x: u8){
        self.origin_x = x;
        self.origin_y = y;
        self.map[y as usize][x as usize] = true;
    }

    /*
    The function take_step takes mutable reference &mut self, meaning that it
    will modify the instance of the struct. The second argument it takes of
    type Direction refers to an enum with enumerators Up, Left, Down, Right.

    In the match statement, it checks which enumerator the direction is.
    Based on the enumerator, it changes the value of self.dx or self.dy,
    which refers to the delta x or delta y shift that is called upon. It
    ensures that the new position does not go out of the screen boundaries.

    Following which, the new x and y positions are calculated by adding the
    corresponding dx or dy values. It then increments the step_count by 1.
    Lastly, it accesses the map field and sets the boolean value at the
    new position to be true, which tracks the path taken.
     */
    pub fn take_step(&mut self, direction: Direction) {
        match direction {
            Direction::Left => if self.origin_x as i16 + self.dx >= 0 { self.dx -= 1},
            Direction::Right => if self.origin_x as i16 + self.dx <= Screen::WIDTH as i16 { self.dx += 1},
            Direction::Down => if self.origin_y as i16 + self.dy <= Screen::HEIGHT as i16 { self.dy += 1},
            Direction::Up => if self.origin_y as i16 + self.dy >= 0 { self.dy -= 1},
        }
        let x = self.origin_x as i16 + self.dx;
        let y = self.origin_y as i16 + self.dy;
        self.step_count += 1;
        self.map[y as usize][x as usize] = true;
    }

    /*
    The function show_positions takes in &mut self as an argument, indicating that
    this function mutates the instance of the object it's called on.

    self.clear(Brightness::WHITE);: It's calling the clear method on self, passing
    Brightness::WHITE as an argument. This method likely resets the screen or browser
    window to a white background.

    let mut x: u8 = 0; let mut y: u8 = 0;: Two mutable variables, x and y, are
    initialized both with value 0 which are coordinates for the screen. Going
    through the outer loop y and inner loop x, it will accesses the map array and
    draw_pixel with brightness black for every boolean value set to be true.
     */
    pub fn show_positions(&mut self) {
        self.clear(Brightness::WHITE);
        let mut x: u8;
        let mut y: u8 = 0;
        while y < Screen::HEIGHT {
            x = 0;
            while x < Screen::WIDTH {
                if self.map[y as usize][x as usize] {
                    self.draw_pixel(x, y, Brightness::BLACK);
                }
                x += 1;
            }
            y += 1;
        }
    }

    /*
    The function show_step_count operates on a mutable reference to instance
    to which the method is applied and method can change the instance.

    self.clear(Brightness::WHITE) calls the clear method of the struct instance
    with an enumerated value Brightness::WHITE as an argument, where the screen
    becomes "clean" with brightness level set to white.

    self.draw_unsigned_int(119, 1, Brightness::BLACK, self.step_count); sets
    the step count at the top right corner of the screen that is 128 x 80 px.
    It takes the data variable step_count and prints them with black pixels.
    */
    pub fn show_step_count(&mut self) {
        self.clear(Brightness::WHITE);
        self.draw_unsigned_int(119, 1, Brightness::BLACK, self.step_count);
    }

    /*
    The function reset_steps takes in &mut self as an argument, indicating that
    this function mutates the instance of the object it's called on and defaults
    the data variables step_count, dy, dx to 0. It also resets the 128 x 80 px
    screen by setting the screen's values to false. It then sets the origin_x
    and origin_y value to be true for the black pixel dot to reset its appearance
    on the white screen.
    */
    pub fn reset_steps(&mut self) {
        self.step_count = 0;
        self.dy = 0;
        self.dx = 0;
        self.map = [[false; Screen::WIDTH as usize]; Screen::HEIGHT as usize];
        self.map[self.origin_y as usize][self.origin_x as usize] = true;
        self.show_positions()
    }

    /*
    The function get_step_count takes a reference to the instance of the struct.
    u32 shows the return type of the function where step_count returns the current
    value of step_count.
    */
    pub fn get_step_count(&mut self) -> u32 {
        self.step_count
    }
}

enum Mode {
    Cmd,
    Data,
}
