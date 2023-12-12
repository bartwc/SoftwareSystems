#![cfg_attr(not(test), no_std)]
#[cfg(test)]
extern crate std;

extern crate alloc;

use alloc::vec::Vec;
use serde::{Serialize, Deserialize};
use crc::{Crc, CRC_32_ISCSI};
use postcard::{from_bytes_cobs, to_allocvec, to_allocvec_cobs};

#[derive(Serialize, Deserialize, PartialEq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Serialize, Deserialize, PartialEq)]
pub enum PayLoad {
    TakeStep(Direction),
    ChangeView,
    Clear,
    StepCountRequest,
    StepCount(u32),
}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct DataFrame {
    pub payload: PayLoad,
}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct Packet {
    dataframe: DataFrame,
    checksum: u32
}

pub fn serialise(data_frame: DataFrame) -> Vec<u8> {
    let crc = Crc::<u32>::new(&CRC_32_ISCSI);
    let checksum_data = crc.checksum(to_allocvec(&data_frame).unwrap().as_slice());
    let packet_to_send = Packet{
        dataframe: data_frame,
        checksum: checksum_data,
    };
    to_allocvec_cobs(&packet_to_send).unwrap()
}

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
