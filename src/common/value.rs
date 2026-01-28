use log::info;
use crate::{common::error::{Exception, Throwable}, utils::u8c::*};

#[derive(Debug, Clone, Copy, PartialEq)]
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
    Null
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

pub fn as_i32(v: &StackFrameValue) -> Result<i32, Throwable> {
    match v {
        StackFrameValue::Int(data) => Ok(*data),
        StackFrameValue::Byte(data) => Ok(*data as i32),
        StackFrameValue::Short(data) => Ok(*data as i32),
        StackFrameValue::CHARACTER(data) => Ok(*data as i32),
        StackFrameValue::Long(data) => {
            if *data < i32::MIN as i64 || *data > i32::MAX as i64 {
                return Err(Throwable::Exception(Exception::Arithmetic(format!("Long value {} out of i32 range", data))));
            }
            Ok(*data as i32)
        },
        StackFrameValue::U32(data) => {
            if *data > i32::MAX as u32 {
                return Err(Throwable::Exception(Exception::Arithmetic(format!("U32 value {} out of i32 range", data))));
            }
            Ok(*data as i32)
        },
        StackFrameValue::Float(_) | StackFrameValue::Double(_) => {
            Err(Throwable::Exception(Exception::IllegalArgument("Cannot convert floating point to i32".to_string())))
        },
        StackFrameValue::Char(data) => Ok(*data as i32),
        StackFrameValue::Boolean(data) => {
            if *data {
                Ok(1)
            } else {
                Ok(0)
            }
        }
        _ => Err(Throwable::Exception(Exception::IllegalArgument(format!("Cannot convert {:?} to i32", v)))),
    }
}

pub fn as_i8(v: &StackFrameValue) -> Result<i8, Throwable> {
    match v {
        StackFrameValue::Byte(data) => Ok(*data),
        StackFrameValue::Int(data) => {
            if *data < i8::MIN as i32 || *data > i8::MAX as i32 {
                return Err(Throwable::Exception(Exception::Arithmetic(format!("Int value {} out of i8 range", data))));
            }
            Ok(*data as i8)
        },
        StackFrameValue::Short(data) => {
            if *data < i8::MIN as i16 || *data > i8::MAX as i16 {
                return Err(Throwable::Exception(Exception::Arithmetic(format!("Short value {} out of i8 range", data))));
            }
            Ok(*data as i8)
        },
        StackFrameValue::Long(data) => {
            if *data < i8::MIN as i64 || *data > i8::MAX as i64 {
                return Err(Throwable::Exception(Exception::Arithmetic(format!("Long value {} out of i8 range", data))));
            }
            Ok(*data as i8)
        },
        StackFrameValue::U32(data) => {
            if *data > i8::MAX as u32 {
                return Err(Throwable::Exception(Exception::Arithmetic(format!("U32 value {} out of i8 range", data))));
            }
            Ok(*data as i8)
        },
        StackFrameValue::Float(_) | StackFrameValue::Double(_) => {
            Err(Throwable::Exception(Exception::IllegalArgument("Cannot convert floating point to i8".to_string())))
        },
        StackFrameValue::Boolean(data) => {
            if *data {
                Ok(1)
            } else {
                Ok(0)
            }
        }
        _ => Err(Throwable::Exception(Exception::IllegalArgument(format!("Cannot convert {:?} to i8", v)))),
    }
}

pub fn as_char(v: &StackFrameValue) -> Result<u16, Throwable> {
    match v {
        StackFrameValue::Char(data) => Ok(*data as u16),
        StackFrameValue::CHARACTER(data) => Ok(*data as u16),
        StackFrameValue::Byte(data) => {
            Ok(*data as u8 as u16)
        },
        StackFrameValue::Short(data) => {
            if *data < 0 || *data > u16::MAX as i16 {
                return Err(Throwable::Exception(Exception::Arithmetic(format!("Short value {} out of char range (0-65535)", data))));
            }
            Ok(*data as u16)
        },
        StackFrameValue::Int(data) => {
            if *data < 0 || *data > u16::MAX as i32 {
                return Err(Throwable::Exception(Exception::Arithmetic(format!("Int value {} out of char range (0-65535)", data))));
            }
            Ok(*data as u16)
        },
        StackFrameValue::U32(data) => {
            if *data > u16::MAX as u32 {
                return Err(Throwable::Exception(Exception::Arithmetic(format!("U32 value {} out of char range (0-65535)", data))));
            }
            Ok(*data as u16)
        },
        StackFrameValue::Float(_) | StackFrameValue::Double(_) => {
            Err(Throwable::Exception(Exception::IllegalArgument("Cannot convert floating point to char".to_string())))
        },
        _ => Err(Throwable::Exception(Exception::IllegalArgument(format!("Cannot convert {:?} to char", v)))),
    }
}

pub fn as_f64(v: &StackFrameValue) -> Result<f64, Throwable> {
    match v {
        StackFrameValue::Double(data) => Ok(*data),
        StackFrameValue::Float(data) => Ok(*data as f64),
        StackFrameValue::Int(data) => Ok(*data as f64),
        StackFrameValue::Byte(data) => Ok(*data as f64),
        StackFrameValue::Short(data) => Ok(*data as f64),
        StackFrameValue::Long(data) => Ok(*data as f64),
        StackFrameValue::U32(data) => Ok(*data as f64),
        _ => Err(Throwable::Exception(Exception::IllegalArgument(format!("Cannot convert {:?} to f64", v)))),
    }
}

pub fn as_f32(v: &StackFrameValue) -> Result<f32, Throwable> {
    match v {
        StackFrameValue::Float(data) => Ok(*data),
        StackFrameValue::Double(data) => Ok(*data as f32),
        StackFrameValue::Int(data) => Ok(*data as f32),
        StackFrameValue::Byte(data) => Ok(*data as f32),
        StackFrameValue::Short(data) => Ok(*data as f32),
        StackFrameValue::Long(data) => Ok(*data as f32),
        StackFrameValue::U32(data) => Ok(*data as f32),
        _ => Err(Throwable::Exception(Exception::IllegalArgument(format!("Cannot convert {:?} to f32", v)))),
    }
}

pub fn as_i64(v: &StackFrameValue) -> Result<i64, Throwable> {
    match v {
        StackFrameValue::Long(data) => Ok(*data),
        StackFrameValue::Int(data) => Ok(*data as i64),
        StackFrameValue::Byte(data) => Ok(*data as i64),
        StackFrameValue::Short(data) => Ok(*data as i64),
        StackFrameValue::U32(data) => Ok(*data as i64),
        StackFrameValue::Float(_) | StackFrameValue::Double(_) => {
            Err(Throwable::Exception(Exception::IllegalArgument("Cannot convert floating point to i64".to_string())))
        },
        _ => Err(Throwable::Exception(Exception::IllegalArgument(format!("Cannot convert {:?} to i64", v)))),
    }
}

pub fn as_i16(v: &StackFrameValue) -> Result<i16, Throwable> {
    match v {
        StackFrameValue::Short(data) => Ok(*data),
        StackFrameValue::Byte(data) => Ok(*data as i16),
        StackFrameValue::Int(data) => {
            if *data < i16::MIN as i32 || *data > i16::MAX as i32 {
                return Err(Throwable::Exception(Exception::Arithmetic(format!("Int value {} out of i16 range", data))));
            }
            Ok(*data as i16)
        },
        StackFrameValue::Long(data) => {
            if *data < i16::MIN as i64 || *data > i16::MAX as i64 {
                return Err(Throwable::Exception(Exception::Arithmetic(format!("Long value {} out of i16 range", data))));
            }
            Ok(*data as i16)
        },
        StackFrameValue::U32(data) => {
            if *data > i16::MAX as u32 {
                return Err(Throwable::Exception(Exception::Arithmetic(format!("U32 value {} out of i16 range", data))));
            }
            Ok(*data as i16)
        },
        StackFrameValue::Float(_) | StackFrameValue::Double(_) => {
            Err(Throwable::Exception(Exception::IllegalArgument("Cannot convert floating point to i16".to_string())))
        },
        StackFrameValue::Char(data) => Ok(*data as i16),
        _ => Err(Throwable::Exception(Exception::IllegalArgument(format!("Cannot convert {:?} to i16", v)))),
    }
}

pub fn as_u32(v: &StackFrameValue) -> Result<u32, Throwable> {
    //info!("as_u32:{:?}", v);
    match v {
        StackFrameValue::U32(data) => Ok(*data),
        StackFrameValue::Byte(data) => {
            if *data < 0 {
                return Err(Throwable::Exception(Exception::Arithmetic(format!("Byte value {} negative for u32", data))));
            }
            Ok(*data as u8 as u32)
        },
        StackFrameValue::Short(data) => {
            if *data < 0 {
                return Err(Throwable::Exception(Exception::Arithmetic(format!("Short value {} negative for u32", data))));
            }
            Ok(*data as u16 as u32)
        },
        StackFrameValue::Int(data) => {
            if *data < 0 {
                return Err(Throwable::Exception(Exception::Arithmetic(format!("Int value {} negative for u32", data))));
            }
            Ok(*data as u32)
        },
        StackFrameValue::Long(data) => {
            if *data < 0 || *data > u32::MAX as i64 {
                return Err(Throwable::Exception(Exception::Arithmetic(format!("Long value {} out of u32 range", data))));
            }
            Ok(*data as u32)
        },
        StackFrameValue::Float(_) | StackFrameValue::Double(_) => {
            Err(Throwable::Exception(Exception::IllegalArgument("Cannot convert floating point to u32".to_string())))
        },
        StackFrameValue::Reference(data) => {
            Ok(*data) 
        },
        StackFrameValue::Null => Ok(0),
        _ => Err(Throwable::Exception(Exception::IllegalArgument(format!("Cannot convert {:?} to u32", v)))),
    }
}