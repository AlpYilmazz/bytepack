use std::array;

use crate::base::{ByteSize, DrainVec};

pub trait ByteUnpack: ByteSize + Sized + 'static {
    fn unpack(buf: &[u8]) -> Result<Self, ()>;
}

impl ByteUnpack for u8 {
    fn unpack(buf: &[u8]) -> Result<Self, ()> {
        Ok(buf[0])
    }
}

// Default is Network (Big Endian) byte order
impl ByteUnpack for u16 {
    fn unpack(buf: &[u8]) -> Result<Self, ()> {
        Ok(Self::from_be_bytes(array::from_fn(|i| buf[i])))
    }
}

// Default is Network (Big Endian) byte order
impl ByteUnpack for u32 {
    fn unpack(buf: &[u8]) -> Result<Self, ()> {
        Ok(Self::from_be_bytes(array::from_fn(|i| buf[i])))
    }
}

// Default is Network (Big Endian) byte order
impl ByteUnpack for u64 {
    fn unpack(buf: &[u8]) -> Result<Self, ()> {
        Ok(Self::from_be_bytes(array::from_fn(|i| buf[i])))
    }
}

// Default is Network (Big Endian) byte order
impl ByteUnpack for u128 {
    fn unpack(buf: &[u8]) -> Result<Self, ()> {
        Ok(Self::from_be_bytes(array::from_fn(|i| buf[i])))
    }
}

macro_rules! imp_unpack_for_le_uXX {
    ($le_u_type: ty, $u_type: ty) => {
        impl ByteUnpack for $le_u_type {
            fn unpack(buf: &[u8]) -> Result<Self, ()> {
                Ok(Self(<$u_type>::from_le_bytes(array::from_fn(|i| buf[i]))))
            }
        }
    };
}

imp_unpack_for_le_uXX!(crate::base::LEu16, u16);
imp_unpack_for_le_uXX!(crate::base::LEu32, u32);
imp_unpack_for_le_uXX!(crate::base::LEu64, u64);
imp_unpack_for_le_uXX!(crate::base::LEu128, u128);

impl<const N: usize> ByteUnpack for [u8; N] {
    fn unpack(buf: &[u8]) -> Result<Self, ()> {
        Ok(array::from_fn(|i| buf[i]))
    }
}

macro_rules! imp_unpack_for_arr_uXX {
    ($u_type: ty, $stride: literal) => {
        impl<const N: usize> ByteUnpack for [$u_type; N] {
            fn unpack(buf: &[u8]) -> Result<Self, ()> {
                Ok(array::from_fn(|i| {
                    <$u_type>::from_be_bytes(array::from_fn(|j| buf[$stride * i + j]))
                }))
            }
        }
    };
}

imp_unpack_for_arr_uXX!(u16, 2);
imp_unpack_for_arr_uXX!(u32, 4);
imp_unpack_for_arr_uXX!(u64, 8);
imp_unpack_for_arr_uXX!(u128, 16);

impl<const N: usize> ByteUnpack for Box<[u8; N]> {
    fn unpack(buf: &[u8]) -> Result<Self, ()> {
        Ok(Box::new(array::from_fn(|i| buf[i])))
    }
}

macro_rules! imp_unpack_for_box_uXX {
    ($u_type: ty, $stride: literal) => {
        impl<const N: usize> ByteUnpack for Box<[$u_type; N]> {
            fn unpack(buf: &[u8]) -> Result<Self, ()> {
                Ok(Box::new(array::from_fn(|i| {
                    <$u_type>::from_be_bytes(array::from_fn(|j| buf[$stride * i + j]))
                })))
            }
        }
    };
}

imp_unpack_for_box_uXX!(u16, 2);
imp_unpack_for_box_uXX!(u32, 4);
imp_unpack_for_box_uXX!(u64, 8);
imp_unpack_for_box_uXX!(u128, 16);

// NOTE: using DrainVec other than the last field is UB
impl<T: ByteUnpack> ByteUnpack for DrainVec<T> {
    fn unpack(buf: &[u8]) -> Result<Self, ()> {
        let mut vec = Vec::new();

        let mut buf = buf;
        while !buf.is_empty() {
            let val_i = T::unpack(buf)?;
            let byte_size = val_i.byte_size();
            buf = &buf[byte_size..];
            vec.push(val_i);
        }

        Ok(Self(vec))
    }
}
