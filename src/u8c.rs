pub fn u8s_to_u16(bytes: &[u8]) -> u16 {
    let mut value = [0; 2];
    value.copy_from_slice(&bytes);
    return Some(u16::from_be_bytes(value)).unwrap();
}

pub fn u8s_to_u32(bytes: &[u8]) -> u32 {
    let mut value = [0; 4];
    value.copy_from_slice(&bytes);
    return Some(u32::from_be_bytes(value)).unwrap();
}

pub fn u8s_to_u64(bytes: &[u8]) -> u64 {
    let mut value = [0; 8];
    value.copy_from_slice(&bytes);
    return Some(u64::from_be_bytes(value)).unwrap();
}

pub fn f64_to_u32_tuple(value: f64) -> (u32, u32) {
    let bits = value.to_bits();
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
