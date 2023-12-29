use std::{cell::RefCell, ops::Deref, rc::Rc, sync::Arc};

use crate::base::{ByteSize, SizeType, SizedVec, SplatVec, ConstByteSize, Throw, SplatDrain};

pub fn pack_value<T: BytePack>(val: &T) -> Result<Vec<u8>, ()> {
    let mut buf_vec = vec![0; val.byte_size()];
    let buf = &mut buf_vec[..];
    let _residue = val.pack(buf)?;
    Ok(buf_vec)
}

pub trait BytePack: ByteSize {
    fn pack(&self, buf: &mut [u8]) -> Result<(), ()>;
}

impl BytePack for u8 {
    fn pack(&self, buf: &mut [u8]) -> Result<(), ()> {
        buf[0] = *self;
        Ok(())
    }
}

// Default is Network (Big Endian) byte order
impl BytePack for u16 {
    fn pack(&self, buf: &mut [u8]) -> Result<(), ()> {
        let be_bytes = self.to_be_bytes();
        for (i, b) in be_bytes.iter().enumerate() {
            buf[i] = *b;
        }
        Ok(())
    }
}

// Default is Network (Big Endian) byte order
impl BytePack for u32 {
    fn pack(&self, buf: &mut [u8]) -> Result<(), ()> {
        let be_bytes = self.to_be_bytes();
        for (i, b) in be_bytes.iter().enumerate() {
            buf[i] = *b;
        }
        Ok(())
    }
}

// Default is Network (Big Endian) byte order
impl BytePack for u64 {
    fn pack(&self, buf: &mut [u8]) -> Result<(), ()> {
        let be_bytes = self.to_be_bytes();
        for (i, b) in be_bytes.iter().enumerate() {
            buf[i] = *b;
        }
        Ok(())
    }
}

// Default is Network (Big Endian) byte order
impl BytePack for u128 {
    fn pack(&self, buf: &mut [u8]) -> Result<(), ()> {
        let be_bytes = self.to_be_bytes();
        for (i, b) in be_bytes.iter().enumerate() {
            buf[i] = *b;
        }
        Ok(())
    }
}

impl BytePack for i8 {
    fn pack(&self, buf: &mut [u8]) -> Result<(), ()> {
        buf[0] = *self as u8;
        Ok(())
    }
}

// Default is Network (Big Endian) byte order
impl BytePack for i16 {
    fn pack(&self, buf: &mut [u8]) -> Result<(), ()> {
        let be_bytes = self.to_be_bytes();
        for (i, b) in be_bytes.iter().enumerate() {
            buf[i] = *b;
        }
        Ok(())
    }
}

// Default is Network (Big Endian) byte order
impl BytePack for i32 {
    fn pack(&self, buf: &mut [u8]) -> Result<(), ()> {
        let be_bytes = self.to_be_bytes();
        for (i, b) in be_bytes.iter().enumerate() {
            buf[i] = *b;
        }
        Ok(())
    }
}

// Default is Network (Big Endian) byte order
impl BytePack for i64 {
    fn pack(&self, buf: &mut [u8]) -> Result<(), ()> {
        let be_bytes = self.to_be_bytes();
        for (i, b) in be_bytes.iter().enumerate() {
            buf[i] = *b;
        }
        Ok(())
    }
}

// Default is Network (Big Endian) byte order
impl BytePack for i128 {
    fn pack(&self, buf: &mut [u8]) -> Result<(), ()> {
        let be_bytes = self.to_be_bytes();
        for (i, b) in be_bytes.iter().enumerate() {
            buf[i] = *b;
        }
        Ok(())
    }
}

macro_rules! imp_pack_for_le_num {
    ($le_u_type: ty) => {
        impl BytePack for $le_u_type {
            fn pack(&self, buf: &mut [u8]) -> Result<(), ()> {
                let le_bytes = self.0.to_le_bytes();
                for (i, b) in le_bytes.iter().enumerate() {
                    buf[i] = *b;
                }
                Ok(())
            }
        }
    };
}

imp_pack_for_le_num!(crate::base::LEu16);
imp_pack_for_le_num!(crate::base::LEu32);
imp_pack_for_le_num!(crate::base::LEu64);
imp_pack_for_le_num!(crate::base::LEu128);
imp_pack_for_le_num!(crate::base::LEi16);
imp_pack_for_le_num!(crate::base::LEi32);
imp_pack_for_le_num!(crate::base::LEi64);
imp_pack_for_le_num!(crate::base::LEi128);

impl<T: BytePack, const N: usize> BytePack for [T; N] {
    fn pack(&self, buf: &mut [u8]) -> Result<(), ()> {
        let mut buf = buf;
        for i in 0..N {
            let val = &self[i];
            let _res = val.pack(buf)?;
            let byte_size = val.byte_size();
            buf = &mut buf[byte_size..];
        }
        Ok(())
    }
}

impl<T: BytePack> BytePack for Box<T> {
    fn pack(&self, buf: &mut [u8]) -> Result<(), ()> {
        self.deref().pack(buf)
    }
}

impl<T: BytePack> BytePack for Rc<T> {
    fn pack(&self, buf: &mut [u8]) -> Result<(), ()> {
        self.deref().pack(buf)
    }
}

impl<T: BytePack> BytePack for RefCell<T> {
    fn pack(&self, buf: &mut [u8]) -> Result<(), ()> {
        self.borrow().pack(buf)
    }
}

impl<T: BytePack> BytePack for Arc<T> {
    fn pack(&self, buf: &mut [u8]) -> Result<(), ()> {
        self.deref().pack(buf)
    }
}

impl<T: BytePack> BytePack for SplatVec<T> {
    fn pack(&self, buf: &mut [u8]) -> Result<(), ()> {
        let mut buf = buf;
        for val in &self.0 {
            let _res = val.pack(buf)?;
            let byte_size = val.byte_size();
            buf = &mut buf[byte_size..];
        }
        Ok(())
    }
}

impl<T: BytePack> BytePack for SplatDrain<T> {
    fn pack(&self, buf: &mut [u8]) -> Result<(), ()> {
        let Self::Splat(vec) = self else {
            return Err(());
        };
        
        let mut buf = buf;
        for val in vec {
            let _res = val.pack(buf)?;
            let byte_size = val.byte_size();
            buf = &mut buf[byte_size..];
        }
        Ok(())
    }
}

impl<T: BytePack> BytePack for SizedVec<T> {
    fn pack(&self, buf: &mut [u8]) -> Result<(), ()> {
        let mut buf = buf;

        let len = self.0.len() as SizeType;
        let _res = len.pack(buf)?;
        buf = &mut buf[len.byte_size()..];

        for val in &self.0 {
            let _res = val.pack(buf)?;
            let byte_size = val.byte_size();
            buf = &mut buf[byte_size..];
        }
        Ok(())
    }
}

impl<T: BytePack + ConstByteSize, const N: usize> BytePack for Throw<T, N> {
    fn pack(&self, _buf: &mut [u8]) -> Result<(), ()> {
        Ok(())
    }
}

impl BytePack for String {
    fn pack(&self, buf: &mut [u8]) -> Result<(), ()> {
        let mut buf = buf;

        let len = self.byte_size() as SizeType;
        let _res = len.pack(buf)?;
        buf = &mut buf[len.byte_size()..];

        (&mut buf[..len as usize]).clone_from_slice(self.as_bytes());

        Ok(())
    }
}
