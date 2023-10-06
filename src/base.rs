#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct LEu16(pub u16);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct LEu32(pub u32);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct LEu64(pub u64);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct LEu128(pub u128);

#[derive(Debug)]
pub struct DrainVec<T>(pub Vec<T>);

#[derive(Debug)]
pub struct SplatVec<T>(pub Vec<T>);

pub trait ByteSize {
    fn byte_size(&self) -> usize;
}

impl ByteSize for u8 {
    fn byte_size(&self) -> usize {
        1
    }
}

impl ByteSize for u16 {
    fn byte_size(&self) -> usize {
        2
    }
}

impl ByteSize for u32 {
    fn byte_size(&self) -> usize {
        4
    }
}

impl ByteSize for u64 {
    fn byte_size(&self) -> usize {
        8
    }
}

impl ByteSize for u128 {
    fn byte_size(&self) -> usize {
        16
    }
}

impl ByteSize for LEu16 {
    fn byte_size(&self) -> usize {
        2
    }
}

impl ByteSize for LEu32 {
    fn byte_size(&self) -> usize {
        4
    }
}

impl ByteSize for LEu64 {
    fn byte_size(&self) -> usize {
        8
    }
}

impl ByteSize for LEu128 {
    fn byte_size(&self) -> usize {
        16
    }
}

impl<const N: usize> ByteSize for [u8; N] {
    fn byte_size(&self) -> usize {
        N
    }
}

impl<const N: usize> ByteSize for [u16; N] {
    fn byte_size(&self) -> usize {
        2 * N
    }
}

impl<const N: usize> ByteSize for [u32; N] {
    fn byte_size(&self) -> usize {
        4 * N
    }
}

impl<const N: usize> ByteSize for [u64; N] {
    fn byte_size(&self) -> usize {
        8 * N
    }
}

impl<const N: usize> ByteSize for [u128; N] {
    fn byte_size(&self) -> usize {
        16 * N
    }
}

impl<const N: usize> ByteSize for Box<[u8; N]> {
    fn byte_size(&self) -> usize {
        N
    }
}

impl<const N: usize> ByteSize for Box<[u16; N]> {
    fn byte_size(&self) -> usize {
        2 * N
    }
}

impl<const N: usize> ByteSize for Box<[u32; N]> {
    fn byte_size(&self) -> usize {
        4 * N
    }
}

impl<const N: usize> ByteSize for Box<[u64; N]> {
    fn byte_size(&self) -> usize {
        8 * N
    }
}

impl<const N: usize> ByteSize for Box<[u128; N]> {
    fn byte_size(&self) -> usize {
        16 * N
    }
}

// TODO: what to do with context dependency
//  - probably should only allow at the end
//    then byte_size does not matter
impl<T: ByteSize> ByteSize for DrainVec<T> {
    fn byte_size(&self) -> usize {
        0
    }
}

impl<T: ByteSize> ByteSize for SplatVec<T> {
    fn byte_size(&self) -> usize {
        if self.0.is_empty() {
            0
        }
        else {
            self.0.len() * self.0[0].byte_size()
        }
    }
}

impl ByteSize for i32 {
    fn byte_size(&self) -> usize {
        4
    }
}