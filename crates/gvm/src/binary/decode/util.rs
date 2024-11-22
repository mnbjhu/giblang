use std::iter::Peekable;

pub fn decode_tiny<T: Iterator<Item = u8>>(iter: &mut Peekable<T>) -> u16 {
    u16::from_be_bytes([iter.next().unwrap(), iter.next().unwrap()])
}

pub fn decode_small<T: Iterator<Item = u8>>(iter: &mut Peekable<T>) -> u32 {
    u32::from_be_bytes([
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
    ])
}

pub fn decode_sign<T: Iterator<Item = u8>>(iter: &mut Peekable<T>) -> i32 {
    i32::from_be_bytes([
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
    ])
}

pub fn decode_big<T: Iterator<Item = u8>>(iter: &mut Peekable<T>) -> u64 {
    u64::from_be_bytes([
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
    ])
}
