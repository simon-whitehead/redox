use std::mem;
use std::ptr;

use super::from_bytes::FromBytes;
use super::block_ptr::BlockPtr;

#[derive(Copy, Clone, Debug)]
#[repr(packed)]
pub struct Uberblock {
    pub magic: u64,
    pub version: u64,
    pub txg: u64,
    pub guid_sum: u64,
    pub timestamp: u64,
    pub rootbp: BlockPtr,
}

impl Uberblock {
    pub fn magic_little() -> u64 {
        return 0x0cb1ba00;
    }

    pub fn magic_big() -> u64 {
        return 0x00bab10c;
    }

}

impl FromBytes for Uberblock {
    fn from_bytes(data: &[u8]) -> Option<Self> {
        if data.len() >= mem::size_of::<Uberblock>() {
            let uberblock = unsafe { ptr::read(data.as_ptr() as *const Uberblock) };
            if uberblock.magic == Uberblock::magic_little() {
                return Some(uberblock);
            } else if uberblock.magic == Uberblock::magic_big() {
                return Some(uberblock);
            }
        }

        None
    }
}
