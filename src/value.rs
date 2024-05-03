pub mod value {
    use log::info;

    use crate::u8c::*;

    #[derive(Debug, Clone)]
    #[derive(PartialEq)]
    pub enum StackFrameValue {
        Byte(i8),
        Char(u16),
        Double(f64),
        Float(f32),
        Int(i32),
        Long(i64),
        Reference(u32),
        Short(i16),
        Boolean(bool),
        U32(u32),
        CHARACTER(char),
        Null,
    }

    pub fn number_to_u32tuple(v: &StackFrameValue) -> (u32, u32){
        let  fv: u64;
        match v {
            StackFrameValue::Int(data) => {
                fv = *data as u64;
            }
            StackFrameValue::Byte(data) => {
                fv = *data as u64;
            }
            StackFrameValue::Char(data) => {
                fv = *data as u64;
            }
            StackFrameValue::Double(data) => {
                return f64_to_u32_tuple(*data);
            }
            StackFrameValue::Float(data) => {
                fv = *data as u64;
            }
            StackFrameValue::Long(data) => {
                fv = *data as u64;
            }
            StackFrameValue::Short(data) => {
                fv = *data as u64;
            }
            _ => {
                panic!("wrong value type");
            }
        }
        u64_to_u32_tuple(fv)
    }


    pub fn number_u64(v: &StackFrameValue) -> u64{
        let  fv: u64;
       // info!("{:?}",v);
        match v {
            StackFrameValue::Int(data) => {
                fv = *data as u64;
            }
            StackFrameValue::Byte(data) => {
                fv = *data as u64;
            }
            StackFrameValue::Char(data) => {
                fv = *data as u64;
            }
            StackFrameValue::Double(data) => {
                fv = *data as u64;
            }
            StackFrameValue::Float(data) => {
                fv = *data as u64;
            }
            StackFrameValue::Long(data) => {
                fv = *data as u64;
            }
            StackFrameValue::Short(data) => {
                fv = *data as u64;
            }
            StackFrameValue::CHARACTER(data) => {
                fv = *data as u64;
            }
            _ => {
                panic!("wrong value type");
            }
        }
        fv
    }


}
