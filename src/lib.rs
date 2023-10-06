pub mod base;
pub mod hex;
pub mod pack;
pub mod unpack;

#[cfg(test)]
mod tests {
    use bytepack_proc_macro::{BytePack, ByteSize, ByteUnpack};

    use super::base::*;
    use super::hex::*;
    use super::pack::*;
    use super::unpack::*;

    #[derive(Debug, ByteSize, ByteUnpack)]
    pub struct TestUnpack {
        pub u8_field: u8,
        pub u32_field: u32,
        pub drain_vec_field: DrainVec<u16>,
    }

    #[derive(Debug, ByteSize, BytePack)]
    pub struct TestPack {
        pub u8_field: u8,
        pub u32_field: LEu32,
        pub splat_vec_field: SplatVec<u8>,
    }

    #[derive(Debug, ByteSize, BytePack, ByteUnpack, PartialEq, Eq)]
    pub struct TestStruct {
        pub u8_field: u8,
        pub leu32_field: LEu32,
        pub arr3_u32_field: [u32; 3],
        pub box2_u16_field: Box<[u16; 2]>,
    }

    fn new_test_struct() -> TestStruct {
        TestStruct {
            u8_field: 0x05,
            leu32_field: LEu32(0xFF00AB08),
            arr3_u32_field: [1, 2, 3],
            box2_u16_field: Box::new([5, 6]),
        }
    }

    #[test]
    fn test_main() {
        let test_struct = new_test_struct();

        dbg!(&test_struct);

        let buf = pack_value(&test_struct).unwrap();

        dbg!(&buf);
        dbg!(buf.into_hex_string());

        let test_unpacked = TestStruct::unpack(&buf).unwrap();

        dbg!(&test_unpacked);

        assert_eq!(test_struct, test_unpacked);
    }

    #[test]
    fn test_pack() {
        let test_pack = TestPack {
            u8_field: 0x05,
            u32_field: LEu32(0xFF00AB08),
            splat_vec_field: SplatVec(vec![
                0x01, 0x02,
                0xFF, 0x00,
            ]),
        };

        let buf = pack_value(&test_pack).unwrap();
        dbg!(&buf);

        let test_unpacked = TestUnpack::unpack(&buf).unwrap();

        dbg!(test_pack);
        dbg!(test_unpacked);
    }

    #[test]
    fn test_unpack() {
        let test_struct = new_test_struct();

        let buf = pack_value(&test_struct).unwrap();

        let test_unpacked = TestUnpack::unpack(&buf).unwrap();

        dbg!(test_struct);
        dbg!(test_unpacked);
    }
}

// impl BytePack for TestStruct {
//     fn pack<'a>(&self, buf: &'a mut [u8]) -> Result<(), ()> {
//         let _res = BytePack::pack(&self.u8_field, buf)?;
//         let byte_size = ByteSize::byte_size(&self.u8_field);
//         let buf = &mut buf[byte_size..];

//         let _res = BytePack::pack(&self.u32_field, buf)?;
//         let byte_size = ByteSize::byte_size(&self.u32_field);
//         let buf = &mut buf[byte_size..];

//         Ok(())
//     }
// }

// impl ByteUnpack for TestStruct {
//     fn unpack(buf: &[u8]) -> Result<Self, ()> {
//         let u8_field = ByteUnpack::unpack(buf)?;
//         let byte_size = ByteSize::byte_size(&u8_field);
//         let buf = &buf[byte_size..];

//         let u32_field = ByteUnpack::unpack(buf)?;
//         let byte_size = ByteSize::byte_size(&u32_field);
//         let buf = &buf[byte_size..];

//         Ok(Self {
//             u8_field,
//             u32_field,
//         })
//     }
// }
