/*
ringbuffer.rs - Address Assignment 3 Part 1 - UART

Requirement - Point 3
You must provide buffering such that long messages can be sent and received.
    - The buffer must be fixed-size, and must be sized appropriately for the messages that are sent over them.
    - If the buffer fills up, the program must not crash.
*/

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

/// The function push_byte takes a mutable reference to an instance of the struct,
/// a byte to push (u8), and returns a Result<(), ()> to indicate whether operation
/// is successful.
///
/// Function checks if the buffer (self.num_elements) is full i.e. > SUZE_BUFFER.
/// If it is full, it returns an error Err(()) as it cannot add more elements.
/// If the buffer is not full, the position at the back of the buffer is computed.
///
/// If position_back is smaller than or equal to buffer size (SIZE_BUFFER -1),
/// position_back is computed using position_front(0) with num_elements (0) and the new byte
/// called (byte_to_push) is placed in the buffer at position (self.data[position_back]).
///
/// If the back position is bigger than buffer size, the buffer is assumed to be
/// circular, and the byte is placed in the buffer at position
/// (self.data[position_back - SIZE_BUFFER]), For every push operation,
/// self.num_elements increments by 1, and concludes by returning Ok(()).
    pub fn push_byte(&mut self, byte_to_push: u8) -> Result<(), ()> {
        if self.num_elements >= SIZE_BUFFER {
            Err(())
        } else {
            let position_back = self.position_front + self.num_elements;
            if position_back <= (SIZE_BUFFER - 1) { // 0 to 255
                self.data[position_back] = byte_to_push;
            } else { // 256 causes reset to 0
                self.data[position_back - SIZE_BUFFER] = byte_to_push;
            }
            self.num_elements = self.num_elements + 1;
            Ok(())
        }
    }

/// The function pop_byte first checks if self.num_elements is less than or equal to 0.
/// If it is (self.num_elements <= 0), the function immediately returns None,
/// indicating that there are no elements to pop from the buffer with safe exit.

/// If it is (self.num_elements > 0), it extracts an element of the (data field)
/// at the index self.position_front, storing it in the variable called data.
/// It then checks if self.position_front is equal to or exceeds the size of the
/// buffer (SIZE_BUFFER - 1). If yes, it means that end of data usize = 256 is reached,
/// it wraps around the buffer by setting self.position_front to 0.

/// If self.position_front is less than SIZE_BUFFER - 1, instead of wrapping,
/// self.position_front increments by 1. For every extracted element,
/// self.num_elements decrements by 1, effectively recording the fact that
/// an element has been popped from the buffer. Finally, it uses Some(element)
/// to return the element it obtained from popping earlier.
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
/// The space_remaining function returns the remaining space of the buffer:
/// it subtracts the current number of elements in the buffer (self.num_elements)
/// from the buffer's total size (SIZE_BUFFER).
    pub fn space_remaining(&mut self) -> usize {
        SIZE_BUFFER - self.num_elements
    }
/// The num_bytes function returns the num_elements attribute of the struct
/// the method was called on. Function returns a usize type, which is a type of
/// integer that can store the size of things in bytes.
    pub fn num_bytes(&mut self) -> usize {
        self.num_elements
    }
}
