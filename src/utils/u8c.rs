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

pub fn split_u64_to_u8(value: u64) -> [u8; 8] {
    [
        ((value >> 56) & 0xFF) as u8,  // 最高字节（第7字节）
        ((value >> 48) & 0xFF) as u8,  // 第6字节
        ((value >> 40) & 0xFF) as u8,  // 第5字节
        ((value >> 32) & 0xFF) as u8,  // 第4字节
        ((value >> 24) & 0xFF) as u8,  // 第3字节
        ((value >> 16) & 0xFF) as u8,  // 第2字节
        ((value >> 8) & 0xFF) as u8,   // 第1字节
        (value & 0xFF) as u8,          // 最低字节（第0字节）
    ]
}


pub fn combine_u8_to_u32(bytes: [u8; 4]) -> u32 {
    (bytes[0] as u32) << 24 |
    (bytes[1] as u32) << 16 |
    (bytes[2] as u32) << 8 |
    (bytes[3] as u32)
}

pub fn split_i32_to_u8(value: i32) -> [u8; 4] {
    [
        ((value >> 24) & 0xFF) as u8,  // 最高字节
        ((value >> 16) & 0xFF) as u8,
        ((value >> 8) & 0xFF) as u8,
        (value & 0xFF) as u8,          // 最低字节
    ]
}


pub fn split_u32_to_u8(value: u32) -> [u8; 4] {
    [
        ((value >> 24) & 0xFF) as u8,  // 最高字节
        ((value >> 16) & 0xFF) as u8,
        ((value >> 8) & 0xFF) as u8,
        (value & 0xFF) as u8,          // 最低字节
    ]
}


pub fn split_i64_to_u8(value: i64) -> [u8; 8] {
    [
        ((value >> 56) & 0xFF) as u8,  // 最高字节
        ((value >> 48) & 0xFF) as u8,
        ((value >> 40) & 0xFF) as u8,
        ((value >> 32) & 0xFF) as u8,
        ((value >> 24) & 0xFF) as u8,
        ((value >> 16) & 0xFF) as u8,
        ((value >> 8) & 0xFF) as u8,
        (value & 0xFF) as u8,          // 最低字节
    ]
}

pub fn split_i16_to_u8(value: i16) -> [u8; 2] {
    [
        ((value >> 8) & 0xFF) as u8,  // 高字节
        (value & 0xFF) as u8,         // 低字节
    ]
}

pub fn split_u16_to_u8(value: u16) -> [u8; 2] {
    [
        ((value >> 8) & 0xFF) as u8,  // 高字节
        (value & 0xFF) as u8,         // 低字节
    ]
}

pub fn decode_java_mutf8(bytes: &[u8]) -> String {
    let mut result = String::new();
    let mut i = 0;
    
    while i < bytes.len() {
        let b1 = bytes[i];
        
        // 1. ASCII 字符 (0x01-0x7F)
        if b1 < 0x80 && b1 != 0 {
            result.push(b1 as char);
            i += 1;
        }
        // 2. Java 特殊 null 编码 (0xC0 0x80)
        else if b1 == 0xC0 && i + 1 < bytes.len() && bytes[i + 1] == 0x80 {
            result.push('\0');
            i += 2;
        }
        // 3. 非法直接 null (应该用 0xC0 0x80 编码)
        else if b1 == 0 {
            result.push('�'); // 非法，替换为替换字符
            i += 1;
        }
        // 4. 双字节 UTF-8 (0xC2-0xDF)
        else if b1 >= 0xC2 && b1 <= 0xDF {
            if i + 1 >= bytes.len() {
                result.push('�');
                break;
            }
            let b2 = bytes[i + 1];
            if b2 < 0x80 || b2 > 0xBF {
                result.push('�');
                i += 1;
                continue;
            }
            let code = ((b1 as u32 & 0x1F) << 6) | (b2 as u32 & 0x3F);
            result.push(std::char::from_u32(code).unwrap_or('�'));
            i += 2;
        }
        // 5. 三字节 UTF-8 (0xE0-0xEF)
        else if b1 >= 0xE0 && b1 <= 0xEF {
            if i + 2 >= bytes.len() {
                result.push('�');
                break;
            }
            let b2 = bytes[i + 1];
            let b3 = bytes[i + 2];
            
            // 检查后续字节有效性
            if b2 < 0x80 || b2 > 0xBF || b3 < 0x80 || b3 > 0xBF {
                result.push('�');
                i += 1;
                continue;
            }
            
            let code = ((b1 as u32 & 0x0F) << 12) 
                     | ((b2 as u32 & 0x3F) << 6) 
                     | (b3 as u32 & 0x3F);
            
            // 检查码点有效性
            if code < 0x800 || (code >= 0xD800 && code <= 0xDFFF) {
                result.push('�');
                i += 3;
                continue;
            }
            
            result.push(std::char::from_u32(code).unwrap_or('�'));
            i += 3;
        }
        // 6. Java 6字节补充字符 (0xED A0-AF + 0xED B0-BF)
        else if b1 == 0xED {
            if i + 5 >= bytes.len() {
                result.push('�');
                break;
            }
            
            // 检查是否是合法的补充字符编码
            let b2 = bytes[i + 1];
            let b3 = bytes[i + 2];
            let b4 = bytes[i + 3];
            let b5 = bytes[i + 4];
            let b6 = bytes[i + 5];
            
            // 格式: ED [A0-AF] [80-BF] ED [B0-BF] [80-BF]
            if b2 >= 0xA0 && b2 <= 0xAF && b3 >= 0x80 && b3 <= 0xBF
                && b4 == 0xED && b5 >= 0xB0 && b5 <= 0xBF && b6 >= 0x80 && b6 <= 0xBF {
                
                // 计算高位代理和低位代理
                let high_surrogate = (((b2 as u32 & 0x0F) << 6) | (b3 as u32 & 0x3F)) as u16;
                let low_surrogate = (((b5 as u32 & 0x0F) << 6) | (b6 as u32 & 0x3F)) as u16;
                
                // 计算 Unicode 码点
                let code_point = 0x10000 + 
                    ((high_surrogate as u32 - 0xD800) << 10) + 
                    (low_surrogate as u32 - 0xDC00);
                
                result.push(std::char::from_u32(code_point).unwrap_or('�'));
                i += 6;
            } else {
                // 不是合法的补充字符编码
                result.push('�');
                i += 1;
            }
        }
        // 7. 其他情况（四字节 UTF-8 在 Java MUTF-8 中应该用6字节编码）
        else if b1 >= 0xF0 && b1 <= 0xF7 {
            // 标准 UTF-8 四字节序列，但在 Java MUTF-8 中不应该出现
            // 安全处理：跳过4字节
            result.push('�');
            i += if i + 3 < bytes.len() { 4 } else { 1 };
        }
        // 8. 完全无效的字节
        else {
            result.push('�');
            i += 1;
        }
    }
    
    result
}