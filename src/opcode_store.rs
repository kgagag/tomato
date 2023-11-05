    use crate::class::ConstantPoolInfo;
    use crate::class::MethodInfo;
    use crate::reference::reference::Reference;
    use crate::runtime_data_area::VM_STACKS;
    use crate::runtime_data_area::create_array;
    use crate::runtime_data_area::pop_stack_frame;
    use crate::runtime_data_area::push_frame_data;
    use crate::stack_frame::StackFrame;
    use crate::stack_frame::init_stack_frame;
    use crate::u8c::f64_to_u32_vec;
    use crate::u8c::i64_to_u32_vec;
    use crate::u8c::u8s_to_u32;
    use std::cell::UnsafeCell;
    use std::collections::HashMap;
    use crate::value::value::StackFrameValue;
    use crate::runtime_data_area::get_class_name;
    use crate::runtime_data_area::get_or_load_class;
    use crate::runtime_data_area::create_object;
    use crate::runtime_data_area::get_method_from_pool;
    extern crate log;
    extern crate env_logger;
    use crate::param::param::MethodParameter;
    use log::{error, info, warn};
    use std::env;


    pub fn fstore(frame: &mut StackFrame) {
        let v = frame.op_stack.pop().unwrap();
        let index = frame.code[frame.pc + 1] as usize;
        frame.local[index] = v;
        frame.pc += 2;
    }

    pub fn astore(frame: &mut StackFrame) {
        let v = frame.op_stack.pop().unwrap();
        let index = frame.code[frame.pc + 1] as usize;
        frame.local[index] = v;
        frame.pc += 2;
    }

    pub fn istore(frame: &mut StackFrame) {
        let v = frame.op_stack.pop().unwrap();
        let index = frame.code[frame.pc + 1] as usize;
        frame.local[index] = v;
        frame.pc += 2;
    }

    
    pub fn lstore(frame: &mut StackFrame) {
        xstore(frame);
    }

    fn xstore(frame: &mut StackFrame){
        let v: StackFrameValue = frame.op_stack.pop().unwrap();
        let index = frame.code[frame.pc + 1] as usize;
        let value:i64;
        match v {
            StackFrameValue::Byte(l) => value = l as i64,
            StackFrameValue::Char(l) => value = l as i64,
            StackFrameValue::Int(l) => value = l as i64,
            StackFrameValue::Short(l) => value = l as i64,
            _=> panic!()
        }
        let bytes: [u8; 8] = value.to_le_bytes();
        let a =  u8s_to_u32(&bytes[0..4]);
        frame.local[index] = StackFrameValue::U32(u8s_to_u32(&bytes[0..4])) ;
        frame.local[index + 1] = StackFrameValue::U32(u8s_to_u32(&bytes[4..8])) ;
        frame.pc += 2;
    }

    pub fn dstore(frame: &mut StackFrame) {
        xstore(frame);
    }

    pub fn istore_0(frame: &mut StackFrame) {
        let i = frame.op_stack.pop().unwrap();
        frame.local[0] = i;
        frame.pc += 1;
    }


    pub fn istore_1(frame: &mut StackFrame) {
        let i = frame.op_stack.pop().unwrap();
        frame.local[1] = i;
        frame.pc += 1;
    }

    pub fn istore_2(frame: &mut StackFrame) {
        let i = frame.op_stack.pop().unwrap();
        frame.local[2] = i;
        frame.pc += 1;
    }

    pub fn istore_3(frame: &mut StackFrame) {
        let i = frame.op_stack.pop().unwrap();
        frame.local[3] = i;
        frame.pc += 1;
    }

    pub fn fstore_0(frame: &mut StackFrame) {
        let i = frame.op_stack.pop().unwrap();
        frame.local[0] = i;
        frame.pc += 1;
    }

    pub fn fstore_1(frame: &mut StackFrame) {
        let i = frame.op_stack.pop().unwrap();
        frame.local[1] = i;
        frame.pc += 1;
    }

    pub fn fstore_2(frame: &mut StackFrame) {
        let i = frame.op_stack.pop().unwrap();
        frame.local[2] = i;
        frame.pc += 1;
    }

    pub fn fstore_3(frame: &mut StackFrame) {
        let i = frame.op_stack.pop().unwrap();
        frame.local[3] = i;
        frame.pc += 1;
    }


    pub fn dstore_0(frame: &mut StackFrame) {
        let v = frame.op_stack.pop().unwrap();
        match v {
            StackFrameValue::Double(d) =>{
              let vs = f64_to_u32_vec(d);
              frame.local[0] = StackFrameValue::U32(*vs.get(0).unwrap());
              frame.local[1] = StackFrameValue::U32(*vs.get(1).unwrap());
            }
            _=> panic!("wrong data type"),
        }
        frame.pc += 1;
    }

    pub fn dstore_1(frame: &mut StackFrame) {
        let v = frame.op_stack.pop().unwrap();
        match v {
            StackFrameValue::Double(d) =>{
              let vs = f64_to_u32_vec(d);
              frame.local[1] = StackFrameValue::U32(*vs.get(0).unwrap());
              frame.local[2] = StackFrameValue::U32(*vs.get(1).unwrap());
            }
            _=> panic!("wrong data type"),
        }
        frame.pc += 1;
    }

    pub fn dstore_2(frame: &mut StackFrame) {
        let v = frame.op_stack.pop().unwrap();
        match v {
            StackFrameValue::Double(d) =>{
              let vs = f64_to_u32_vec(d);
              frame.local[2] = StackFrameValue::U32(*vs.get(0).unwrap());
              frame.local[3] = StackFrameValue::U32(*vs.get(1).unwrap());
            }
            _=> panic!("wrong data type"),
        }
        frame.pc += 1;
    }

    pub fn dstore_3(frame: &mut StackFrame) {
        let v = frame.op_stack.pop().unwrap();
        match v {
            StackFrameValue::Double(d) =>{
              let vs = f64_to_u32_vec(d);
              frame.local[3] = StackFrameValue::U32(*vs.get(0).unwrap());
              frame.local[4] = StackFrameValue::U32(*vs.get(1).unwrap());
            }
            _=> panic!("wrong data type"),
        }
        frame.pc += 1;
    }


    pub fn lstore_0(frame: &mut StackFrame) {
        let v = frame.op_stack.pop().unwrap();
        match v {
            StackFrameValue::Long(d) =>{
              let vs = i64_to_u32_vec(d);
              frame.local[0] = StackFrameValue::U32(*vs.get(0).unwrap());
              frame.local[1] = StackFrameValue::U32(*vs.get(1).unwrap());
            }
            _=> panic!("wrong data type"),
        }
        frame.pc += 1;
    }

    pub fn lstore_1(frame: &mut StackFrame) {
        let v = frame.op_stack.pop().unwrap();
        match v {
            StackFrameValue::Long(d) =>{
              let vs = i64_to_u32_vec(d);
              frame.local[1] = StackFrameValue::U32(*vs.get(0).unwrap());
              frame.local[2] = StackFrameValue::U32(*vs.get(1).unwrap());
            }
            _=> panic!("wrong data type"),
        }
        frame.pc += 1;
    }

    pub fn lstore_2(frame: &mut StackFrame) {
        let v = frame.op_stack.pop().unwrap();
        match v {
            StackFrameValue::Long(d) =>{
              let vs = i64_to_u32_vec(d);
              frame.local[2] = StackFrameValue::U32(*vs.get(0).unwrap());
              frame.local[3] = StackFrameValue::U32(*vs.get(1).unwrap());
            }
            _=> panic!("wrong data type"),
        }
        frame.pc += 1;
    }

    pub fn lstore_3(frame: &mut StackFrame) {
        let v = frame.op_stack.pop().unwrap();
        match v {
            StackFrameValue::Long(d) =>{
              let vs = i64_to_u32_vec(d);
              frame.local[3] = StackFrameValue::U32(*vs.get(0).unwrap());
              frame.local[4] = StackFrameValue::U32(*vs.get(1).unwrap());
            }
            _=> panic!("wrong data type"),
        }
        frame.pc += 1;
    }

    pub fn astore_0(frame: &mut StackFrame) {
        let i = frame.op_stack.pop().unwrap();
        frame.local[0] = i;
        frame.pc += 1;
    }

    pub fn astore_1(frame: &mut StackFrame) {
        let i = frame.op_stack.pop().unwrap();
        frame.local[1] = i;
        frame.pc += 1;
    }

    pub fn astore_2(frame: &mut StackFrame) {
        let i = frame.op_stack.pop().unwrap();
        frame.local[2] = i;
        frame.pc += 1;
    }
    
    pub fn astore_3(frame: &mut StackFrame) {
        let i = frame.op_stack.pop().unwrap();
        frame.local[3] = i;
        frame.pc += 1;
    }
