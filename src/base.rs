use std::{cell::RefCell, ops::{Deref, DerefMut}, rc::Rc, marker::PhantomData, sync::Arc};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct LEu16(pub u16);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct LEu32(pub u32);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct LEu64(pub u64);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct LEu128(pub u128);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct LEi16(pub i16);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct LEi32(pub i32);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct LEi64(pub i64);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct LEi128(pub i128);

#[derive(Debug)]
pub struct SplatVec<T>(pub Vec<T>);

#[derive(Debug)]
pub struct DrainVec<T>(pub Vec<T>);

#[derive(Debug)]
pub enum SplatDrain<T> {
    Splat(Vec<T>),
    Drain(Vec<T>),
}
impl<T> SplatDrain<T> {
    pub fn into_vec(self) -> Vec<T> {
        match self {
            Self::Splat(v) => v,
            Self::Drain(v) => v,
        }
    }

    pub fn into_splat(self) -> Self {
        let v = self.into_vec();
        Self::Splat(v)
    }

    pub fn into_drain(self) -> Self {
        let v = self.into_vec();
        Self::Drain(v)
    }
}

#[derive(Debug)]
pub struct SizedVec<T>(pub Vec<T>);
pub type SizeType = u32;

#[derive(Debug)]
pub struct Throw<T, const N: usize>(PhantomData<fn() -> T>);
impl<T, const N: usize> Throw<T, N> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

pub trait ConstByteSize {
    fn const_byte_size() -> usize;
}

pub trait ByteSize {
    fn byte_size(&self) -> usize;
}

macro_rules! imp_const_bytesize {
    ($num_type: ty, $byte_size: literal) => {
        impl ConstByteSize for $num_type {
            fn const_byte_size() -> usize {
                $byte_size
            }
        }
        impl ByteSize for $num_type {
            fn byte_size(&self) -> usize {
                Self::const_byte_size()
            }
        }
    };
}

imp_const_bytesize!(u8, 1);
imp_const_bytesize!(u16, 2);
imp_const_bytesize!(u32, 4);
imp_const_bytesize!(u64, 8);
imp_const_bytesize!(u128, 16);
imp_const_bytesize!(LEu16, 2);
imp_const_bytesize!(LEu32, 4);
imp_const_bytesize!(LEu64, 8);
imp_const_bytesize!(LEu128, 16);
imp_const_bytesize!(i8, 1);
imp_const_bytesize!(i16, 2);
imp_const_bytesize!(i32, 4);
imp_const_bytesize!(i64, 8);
imp_const_bytesize!(i128, 16);
imp_const_bytesize!(LEi16, 2);
imp_const_bytesize!(LEi32, 4);
imp_const_bytesize!(LEi64, 8);
imp_const_bytesize!(LEi128, 16);

impl<T: ByteSize, const N: usize> ByteSize for [T; N] {
    fn byte_size(&self) -> usize {
        self.iter().map(ByteSize::byte_size).sum()
    }
}

impl<T: ByteSize> ByteSize for Box<T> {
    fn byte_size(&self) -> usize {
        self.deref().byte_size()
    }
}

impl<T: ByteSize> ByteSize for Rc<T> {
    fn byte_size(&self) -> usize {
        self.deref().byte_size()
    }
}

impl<T: ByteSize> ByteSize for RefCell<T> {
    fn byte_size(&self) -> usize {
        self.borrow().byte_size()
    }
}

impl<T: ByteSize> ByteSize for Arc<T> {
    fn byte_size(&self) -> usize {
        self.deref().byte_size()
    }
}

impl<T: ByteSize> ByteSize for SplatVec<T> {
    fn byte_size(&self) -> usize {
        self.0.iter().map(ByteSize::byte_size).sum()
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

impl<T: ByteSize> ByteSize for SplatDrain<T> {
    fn byte_size(&self) -> usize {
        match self {
            Self::Splat(v) => v.iter().map(ByteSize::byte_size).sum(),
            Self::Drain(_v) => 0,
        }
    }
}

impl<T: ByteSize> ByteSize for SizedVec<T> {
    fn byte_size(&self) -> usize {
        self.0.iter().map(ByteSize::byte_size).sum()
    }
}

impl<T: ConstByteSize, const N: usize> ConstByteSize for Throw<T, N> {
    fn const_byte_size() -> usize {
        T::const_byte_size() * N
    }
}
impl<T: ConstByteSize, const N: usize> ByteSize for Throw<T, N> {
    fn byte_size(&self) -> usize {
        <Self as ConstByteSize>::const_byte_size()
    }
}

impl ByteSize for String {
    fn byte_size(&self) -> usize {
        self.len()
    }
}

impl<T> Deref for SplatVec<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for SplatVec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> Deref for DrainVec<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for DrainVec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> Deref for SplatDrain<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Splat(v) => v,
            Self::Drain(v) => v,
        }
    }
}

impl<T> DerefMut for SplatDrain<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Self::Splat(v) => v,
            Self::Drain(v) => v,
        }
    }
}

impl<T> Deref for SizedVec<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for SizedVec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}