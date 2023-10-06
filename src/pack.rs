use crate::base::{ByteSize, SplatVec};

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

macro_rules! imp_pack_for_le_uXX {
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

imp_pack_for_le_uXX!(crate::base::LEu16);
imp_pack_for_le_uXX!(crate::base::LEu32);
imp_pack_for_le_uXX!(crate::base::LEu64);
imp_pack_for_le_uXX!(crate::base::LEu128);

impl<const N: usize> BytePack for [u8; N] {
    fn pack(&self, buf: &mut [u8]) -> Result<(), ()> {
        for i in 0..N {
            buf[i] = self[i];
        }
        Ok(())
    }
}

impl<const N: usize> BytePack for Box<[u8; N]> {
    fn pack(&self, buf: &mut [u8]) -> Result<(), ()> {
        for i in 0..N {
            buf[i] = self[i];
        }
        Ok(())
    }
}

macro_rules! imp_pack_for_slice_uXX {
    ($slice_u_type: ty) => {
        impl<const N: usize> BytePack for $slice_u_type {
            fn pack(&self, buf: &mut [u8]) -> Result<(), ()> {
                let mut buf = buf;
                for i in 0..N {
                    let val = self[i];
                    let _res = val.pack(buf)?;
                    let byte_size = val.byte_size();
                    buf = &mut buf[byte_size..];
                }
                Ok(())
            }
        }
    };
}

imp_pack_for_slice_uXX!([u16; N]);
imp_pack_for_slice_uXX!([u32; N]);
imp_pack_for_slice_uXX!([u64; N]);
imp_pack_for_slice_uXX!([u128; N]);
imp_pack_for_slice_uXX!(Box<[u16; N]>);
imp_pack_for_slice_uXX!(Box<[u32; N]>);
imp_pack_for_slice_uXX!(Box<[u64; N]>);
imp_pack_for_slice_uXX!(Box<[u128; N]>);

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