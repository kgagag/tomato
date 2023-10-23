pub mod u8c {
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
}
