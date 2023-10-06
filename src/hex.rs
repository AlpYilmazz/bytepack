
#[derive(Debug)]
pub enum HexStringError {
    InvalidHexChar(u8)
}

pub fn hexchar_repr(b: &u8) -> Result<char, HexStringError>  {
    match b {
        0 => Ok('0'),
        1 => Ok('1'),
        2 => Ok('2'),
        3 => Ok('3'),
        4 => Ok('4'),
        5 => Ok('5'),
        6 => Ok('6'),
        7 => Ok('7'),
        8 => Ok('8'),
        9 => Ok('9'),
        10 => Ok('a'),
        11 => Ok('b'),
        12 => Ok('c'),
        13 => Ok('d'),
        14 => Ok('e'),
        15 => Ok('f'),
        _ => Err(HexStringError::InvalidHexChar(*b)),
    }
}

/// Convert a byte to its two-character hex string representation
pub fn u8_to_hex_string(b: &u8) -> [char; 2] {
    let upper = hexchar_repr(&((b & 0xf0) >> 4)).expect("Invalid cannot occur");
    let lower = hexchar_repr(&(b & 0x0f)).expect("Invalid cannot occur");
    [upper, lower]
}

pub fn encode_as_hex_string(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|b| u8_to_hex_string(b))
        .fold(String::new(), |mut acc, s| {
            acc.push(s[0]);
            acc.push(s[1]);
            acc
        })
}

pub trait IntoHexString {
    fn into_hex_string(&self) -> String;
}

impl IntoHexString for [u8] {
    fn into_hex_string(&self) -> String {
        encode_as_hex_string(self)
    }
}