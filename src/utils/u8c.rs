use log::info;

pub fn u8s_to_u16(bytes: &[u8]) -> u16 {
    u16::from_be_bytes(bytes.try_into().unwrap())
}

pub fn u8s_to_i16(bytes: &[u8]) -> i16 {
    i16::from_be_bytes(bytes.try_into().unwrap())
}

pub fn u8s_to_u32(bytes: &[u8]) -> u32 {
    u32::from_be_bytes(bytes.try_into().unwrap())
}

pub fn u8s_to_u64(bytes: &[u8]) -> u64 {
    u64::from_be_bytes(bytes.try_into().unwrap())
}

pub fn f64_to_u32_tuple(v: f64) -> (u32, u32) {
    let bits = v.to_bits();
    let lower = bits as u32;
    let upper: u32 = (bits >> 32) as u32;
    (lower, upper)
}

pub fn u64_to_u32_tuple(bits: u64) -> (u32, u32) {
    let lower = bits as u32;
    let upper = (bits >> 32) as u32;
    (lower, upper)
}

pub fn u32_tuple_to_f64(tuple: (u32, u32)) -> f64 {
    let bits = ((tuple.1 as u64) << 32) | (tuple.0 as u64);
    f64::from_bits(bits) 
}


pub fn i64_to_u32_tuple(value: i64) -> (u32, u32) {
    let lower = value as u32;
    let upper = (value >> 32) as u32;
    (lower, upper)
}

pub fn u32_tuple_to_i64(tuple: (u32, u32)) -> i64 {
    let lower = tuple.0 as u64;
    let upper = (tuple.1 as u64) << 32;
    (lower | upper) as i64
}

pub fn u32_tuple_to_u64(tuple: (u32, u32)) -> u64 {
    let lower = tuple.0 as u64;
    let upper = (tuple.1 as u64) << 32;
    lower | upper
}


pub fn char_to_bytes(c: char) -> Vec<u8> {
    //c.encode_utf8(dst)
    let mut buffer = [0; 4];
    let encoded = c.encode_utf8(&mut buffer);
    encoded.as_bytes().to_vec()
}

pub fn bytes_to_chars(bytes: Vec<u8>) -> Vec<char> {
    let string = String::from_utf8(bytes).expect("Invalid UTF-8 sequence");
    //info!("{:?}",string);
    string.chars().collect()
}