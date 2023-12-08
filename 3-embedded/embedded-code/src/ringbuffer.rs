const SIZE_BUFFER: usize = 256;

pub struct RingBuffer {
    data: [u8; SIZE_BUFFER],
    num_elements: usize,
    position_front: usize,
}

impl RingBuffer {
    pub fn new() -> Self {
        Self {
            data: [0; SIZE_BUFFER],
            num_elements: 0,
            position_front: 0,
        }
    }

    pub fn pop_byte(&mut self) -> Option<u8> {
        if self.num_elements <= 0 {
            None
        } else {
            let element = self.data[self.position_front];
            if self.position_front >= (SIZE_BUFFER - 1)
            {
                self.position_front = 0;
            } else {
                self.position_front = self.position_front + 1;
            }
            self.num_elements = self.num_elements - 1;
            Some(element)
        }
    }

    pub fn push_byte(&mut self, byte_to_push: u8) -> Result<(), ()> {
        if self.num_elements >= SIZE_BUFFER {
            Err(())
        } else {
            let position_back = self.position_front + self.num_elements;
            if position_back <= (SIZE_BUFFER - 1) {
                self.data[position_back] = byte_to_push;
            } else {
                self.data[position_back - SIZE_BUFFER] = byte_to_push;
            }
            self.num_elements = self.num_elements + 1;
            Ok(())
        }
    }
}
