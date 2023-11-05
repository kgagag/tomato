
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

    pub fn f64_to_u32_vec(f64_value: f64) -> Vec<u32> {
        // 将 f64 转换为 8 个 u8 字节
        let bytes: [u8; 8] = unsafe { std::mem::transmute(f64_value) };
    
        // 分别将前4个字节和后4个字节转换为两个 u32 值
        let u32_part1: u32 = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        let u32_part2: u32 = u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
    
        let mut result = Vec::new();
        result.push(u32_part1);
        result.push(u32_part2);
    
        result
    }

    pub fn i64_to_u32_vec(i64_value: i64) -> Vec<u32> {
        // 将 f64 转换为 8 个 u8 字节
        let bytes: [u8; 8] = unsafe { std::mem::transmute(i64_value) };
    
        // 分别将前4个字节和后4个字节转换为两个 u32 值
        let u32_part1: u32 = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        let u32_part2: u32 = u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
    
        let mut result = Vec::new();
        result.push(u32_part1);
        result.push(u32_part2);
    
        result
    }
    

