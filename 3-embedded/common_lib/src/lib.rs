#![cfg_attr(not(test), no_std)]
#[cfg(test)]
extern crate std;

extern crate alloc;

use alloc::vec::Vec;
use serde::{Serialize, Deserialize};
use crc::{Crc, CRC_32_ISCSI};
use postcard::{from_bytes_cobs, to_allocvec, to_allocvec_cobs};

// Given the provided Direction enum definition, the variant
// Direction::Left is likely used to signify a request to
// retrieve the Left Task. It also can have other request
// like Right, Up, and Down.
#[derive(Serialize, Deserialize, PartialEq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}
// Given the provided PayLoad enum definition, the variant
// PayLoad::StepCountRequest is likely used to signify a request to
// retrieve the current step count. It also can have other request
// like TakeStep (Direction), ChangeView (of Page), Clear (steps),
// and StepCount.
#[derive(Serialize, Deserialize, PartialEq)]
pub enum PayLoad {
    TakeStep(Direction),
    ChangeView,
    Clear,
    StepCountRequest,
    StepCount(u32),
}

// The DataFrame struct, which includes a payload field of type PayLoad,
// is used to encapsulate this request as part of a message that can be
// serialized and sent over a communication stream.
#[derive(Serialize, Deserialize, PartialEq)]
pub struct DataFrame {
    pub payload: PayLoad,
}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct Packet {
    dataframe: DataFrame,
    checksum: u32
}

/*
The serialise function takes a DataFrame as input and returns a vector of
bytes (Vec<u8>) after performing serialization.

It creates a new cyclic redundancy check (CRC) instance using the Crc type
from the crc crate, with a specific configuration CRC_32_ISCSI.
It calculates the checksum for the data_frame by invoking the checksum method
on the CRC instance. The to_allocvec function is used to convert the data_frame
into an allocatable vector, and unwrap() is used to assert that the conversion
succeeds. The checksum is computed based on the data from the DataFrame.

It creates a Packet to be sent, which includes the original data_frame and its
checksum. It then serialises the Packet using COBS (Consistent Overhead Byte
Stuffing) encoding by invoking the to_allocvec_cobs function. The result is
unwrapped to obtain the vector of bytes to be sent over the communication stream.
 */
pub fn serialise(data_frame: DataFrame) -> Vec<u8> {
    let crc = Crc::<u32>::new(&CRC_32_ISCSI);
    let checksum_data = crc.checksum(to_allocvec(&data_frame).unwrap().as_slice());
    let packet_to_send = Packet{
        dataframe: data_frame,
        checksum: checksum_data,
    };
    to_allocvec_cobs(&packet_to_send).unwrap()
}

/*
The deserialise function takes a mutable slice of bytes (&mut [u8]) as input
and attempts to deserialize it into a DataFrame.

It creates a new cyclic redundancy check (CRC) instance using the Crc type from
the crc crate, with a specific configuration CRC_32_ISCSI. It deserializes the
input byte slice into a Packet using COBS (Consistent Overhead Byte Stuffing)
decoding by invoking the from_bytes_cobs function. The result is pattern matched
using if let to handle the successful case where a Packet is obtained from the
byte slice.

If deserialization is successful (i.e., if a Packet is obtained from the byte slice),
it calculates the checksum for the deserialized Packet and compares it with the
checksum included in the deserialized Packet. If the checksums match, it returns
Some(packet.dataframe) as an Option<DataFrame>, indicating that the deserialization
was successful and providing the deserialized DataFrame. If the checksums do not match,
it returns None, indicating that the deserialization failed due to a checksum mismatch.
*/
pub fn deserialise(byte_slice: & mut [u8]) -> Option<DataFrame> {
    let crc = Crc::<u32>::new(&CRC_32_ISCSI);
    if let Ok(packet) = from_bytes_cobs::<Packet>(byte_slice) {
        let checksum_data = crc.checksum(to_allocvec(&(packet.dataframe)).unwrap().as_slice());
        if checksum_data == packet.checksum {
            Some(packet.dataframe)
        }
        else { None }
    }
    else {
        None
    }
}

/*
The add function takes two arguments of type usize, adds them together, and returns
the result as a usize. This function essentially performs addition on two numbers and
returns the sum.
 */
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
