use log::info;

use crate::utils::u8c::*;

#[derive(Debug, Clone, Copy,PartialEq)]
pub enum StackFrameValue {
    Byte(i8),
    Char(i16),
    Double(f64),
    Float(f32),
    Int(i32),
    Long(i64),
    Short(i16),
    Reference(u32),
    Boolean(bool),
    U32(u32),
    U64(u64),
    CHARACTER(char),
    Null,
}

pub fn number_to_u32tuple(v: &StackFrameValue) -> (u32, u32) {
    let fv = match v {
        StackFrameValue::Int(data) => *data as u64,
        StackFrameValue::Byte(data) => *data as u64,
        StackFrameValue::Char(data) => *data as u64,
        StackFrameValue::Double(data) => return f64_to_u32_tuple(*data),
        StackFrameValue::Float(data) => *data as u64,
        StackFrameValue::Long(data) => *data as u64,
        StackFrameValue::Short(data) => *data as u64,
        StackFrameValue::U64(data) => *data,
        StackFrameValue::U32(data) => *data as u64,
        _ => panic!("wrong value type"),
    };
    u64_to_u32_tuple(fv)
}


pub fn number_u64(v: &StackFrameValue) -> u64 {
    match v {
        StackFrameValue::Int(data) => *data as u64,
        StackFrameValue::Byte(data) => *data as u64,
        StackFrameValue::Char(data) => *data as u64,
        StackFrameValue::Double(data) => *data as u64,
        StackFrameValue::Float(data) => *data as u64,
        StackFrameValue::Long(data) => *data as u64,
        StackFrameValue::Short(data) => *data as u64,
        StackFrameValue::CHARACTER(data) => *data as u64,
        StackFrameValue::U32(data) => *data as u64,
        _ => panic!("wrong value type"),
    }
}

pub fn as_i32(v: &StackFrameValue) -> i32 {
    match v {
        StackFrameValue::Int(data) => *data,
        _ => panic!("Expected Int, got {:?}", v),
    }
}

pub fn as_i8(v: &StackFrameValue) -> i8 {
    match v {
        StackFrameValue::Byte(data) => *data,
        _ => panic!("Expected Byte, got {:?}", v),
    }
}

pub fn as_char(v: &StackFrameValue) -> char {
    match v {
        StackFrameValue::Char(data) => *data,
        StackFrameValue::CHARACTER(data) => *data,
        _ => panic!("Expected Char, got {:?}", v),
    }
}

pub fn as_f64(v: &StackFrameValue) -> f64 {
    match v {
        StackFrameValue::Double(data) => *data,
        _ => panic!("Expected Double, got {:?}", v),
    }
}

pub fn as_f32(v: &StackFrameValue) -> f32 {
    match v {
        StackFrameValue::Float(data) => *data,
        _ => panic!("Expected Float, got {:?}", v),
    }
}

pub fn as_i64(v: &StackFrameValue) -> i64 {
    match v {
        StackFrameValue::Long(data) => *data,
        _ => panic!("Expected Long, got {:?}", v),
    }
}

pub fn as_i16(v: &StackFrameValue) -> i16 {
    match v {
        StackFrameValue::Short(data) => *data,
        _ => panic!("Expected Short, got {:?}", v),
    }
}

pub fn as_u32(v: &StackFrameValue) -> u32 {
    match v {
        StackFrameValue::U32(data) => *data,
        _ => panic!("Expected U32, got {:?}", v),
    }
}