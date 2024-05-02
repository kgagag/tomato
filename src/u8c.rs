pub fn u8s_to_u16(bytes: &[u8]) -> u16 {
    let mut value: [u8; 2] = [0; 2];
    value.copy_from_slice(&bytes);
    u16::from_be_bytes(value)
}

pub fn u8s_to_u32(bytes: &[u8]) -> u32 {
    let mut value = [0; 4];
    value.copy_from_slice(&bytes);
    u32::from_be_bytes(value)
}

pub fn u8s_to_u64(bytes: &[u8]) -> u64 {
    let mut value = [0; 8];
    value.copy_from_slice(&bytes);
    u64::from_be_bytes(value)
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

// fn combine_u32_to_f64(high: u32, low: u32) -> f64 {
//     let bits = ((high as u64) << 32) | (low as u64); // 将两个u32组合成一个64位整数
//     f64::from_bits(bits) // 将64位整数转换回f64
// }

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
