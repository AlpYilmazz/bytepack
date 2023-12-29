use std::array;
use std::sync::Arc;
use std::{cell::RefCell, rc::Rc};

use crate::base::{ByteSize, ConstByteSize, DrainVec, SizedVec, SizeType, Throw, SplatDrain};

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

impl ByteUnpack for i8 {
    fn unpack(buf: &[u8]) -> Result<Self, ()> {
        Ok(buf[0] as i8)
    }
}

// Default is Network (Big Endian) byte order
impl ByteUnpack for i16 {
    fn unpack(buf: &[u8]) -> Result<Self, ()> {
        Ok(Self::from_be_bytes(array::from_fn(|i| buf[i])))
    }
}

// Default is Network (Big Endian) byte order
impl ByteUnpack for i32 {
    fn unpack(buf: &[u8]) -> Result<Self, ()> {
        Ok(Self::from_be_bytes(array::from_fn(|i| buf[i])))
    }
}

// Default is Network (Big Endian) byte order
impl ByteUnpack for i64 {
    fn unpack(buf: &[u8]) -> Result<Self, ()> {
        Ok(Self::from_be_bytes(array::from_fn(|i| buf[i])))
    }
}

// Default is Network (Big Endian) byte order
impl ByteUnpack for i128 {
    fn unpack(buf: &[u8]) -> Result<Self, ()> {
        Ok(Self::from_be_bytes(array::from_fn(|i| buf[i])))
    }
}

macro_rules! imp_unpack_for_le_num {
    ($le_u_type: ty, $u_type: ty) => {
        impl ByteUnpack for $le_u_type {
            fn unpack(buf: &[u8]) -> Result<Self, ()> {
                Ok(Self(<$u_type>::from_le_bytes(array::from_fn(|i| buf[i]))))
            }
        }
    };
}

imp_unpack_for_le_num!(crate::base::LEu16, u16);
imp_unpack_for_le_num!(crate::base::LEu32, u32);
imp_unpack_for_le_num!(crate::base::LEu64, u64);
imp_unpack_for_le_num!(crate::base::LEu128, u128);
imp_unpack_for_le_num!(crate::base::LEi16, i16);
imp_unpack_for_le_num!(crate::base::LEi32, i32);
imp_unpack_for_le_num!(crate::base::LEi64, i64);
imp_unpack_for_le_num!(crate::base::LEi128, i128);

impl<T: ByteUnpack + ConstByteSize, const N: usize> ByteUnpack for [T; N] {
    fn unpack(buf: &[u8]) -> Result<Self, ()> {
        let stride = <T as ConstByteSize>::const_byte_size();
        let arr = array::from_fn(|i| <T as ByteUnpack>::unpack(&buf[stride * i..]));
        for e in &arr {
            if e.is_err() {
                Err(())?
            }
        }
        Ok(arr.map(|e| e.unwrap()))
    }
}

impl<T: ByteUnpack> ByteUnpack for Box<T> {
    fn unpack(buf: &[u8]) -> Result<Self, ()> {
        Ok(Box::new(<T as ByteUnpack>::unpack(buf)?))
    }
}

impl<T: ByteUnpack> ByteUnpack for Rc<T> {
    fn unpack(buf: &[u8]) -> Result<Self, ()> {
        Ok(Rc::new(<T as ByteUnpack>::unpack(buf)?))
    }
}

impl<T: ByteUnpack> ByteUnpack for RefCell<T> {
    fn unpack(buf: &[u8]) -> Result<Self, ()> {
        Ok(RefCell::new(<T as ByteUnpack>::unpack(buf)?))
    }
}

impl<T: ByteUnpack> ByteUnpack for Arc<T> {
    fn unpack(buf: &[u8]) -> Result<Self, ()> {
        Ok(Arc::new(<T as ByteUnpack>::unpack(buf)?))
    }
}

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

impl<T: ByteUnpack> ByteUnpack for SplatDrain<T> {
    fn unpack(buf: &[u8]) -> Result<Self, ()> {
        Ok(Self::Drain(DrainVec::unpack(buf)?.0))
    }
}

impl<T: ByteUnpack> ByteUnpack for SizedVec<T> {
    fn unpack(buf: &[u8]) -> Result<Self, ()> {
        let mut buf = buf;

        let len = SizeType::unpack(buf)?;
        buf = &buf[len.byte_size()..];

        let mut vec = Vec::with_capacity(len as usize);

        for _ in 0..len {
            let val_i = T::unpack(buf)?;
            let byte_size = val_i.byte_size();
            buf = &buf[byte_size..];
            vec.push(val_i);
        }

        Ok(Self(vec))
    }
}

impl<T: ByteUnpack + ConstByteSize, const N: usize> ByteUnpack for Throw<T, N> {
    fn unpack(_buf: &[u8]) -> Result<Self, ()> {
        Ok(Self::new())
    }
}

impl ByteUnpack for String {
    fn unpack(buf: &[u8]) -> Result<Self, ()> {
        let mut buf = buf;

        let len = SizeType::unpack(buf)?;
        buf = &buf[len.byte_size()..];

        Ok(String::from_utf8((&buf[..len as usize]).to_owned()).map_err(|_| ())?)
    }
}
