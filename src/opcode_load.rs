
use crate::stack_frame::StackFrame;
use crate::u8c::*;


use crate::value::value::StackFrameValue;

extern crate log;
extern crate env_logger;
use log::{error, info, warn};
pub fn aload_1(frame: &mut StackFrame) {
    frame.op_stack.push(frame.local.get(1).unwrap().clone());
    frame.pc += 1;
}

pub fn aload_2(frame: &mut StackFrame) {
    frame.op_stack.push(frame.local.get(2).unwrap().clone());
    frame.pc += 1;
}

pub fn aload_3(frame: &mut StackFrame) {
    frame.op_stack.push(frame.local.get(3).unwrap().clone());
    frame.pc += 1;
}

pub fn aload_0(frame: &mut StackFrame) {
    frame.op_stack.push(frame.local.get(0).unwrap().clone());
    frame.pc += 1;
}

pub fn iload(frame: &mut StackFrame) {
    let index = frame.code[frame.pc + 1] as usize;
    frame.op_stack.push(frame.local.get(index).unwrap().clone());
    frame.pc += 2;
}


pub fn fload(frame: &mut StackFrame) {
    let index = frame.code[frame.pc + 1] as usize;
    frame.op_stack.push(frame.local.get(index).unwrap().clone());
    frame.pc += 2;
}

pub fn aload(frame: &mut StackFrame) {
    let index = frame.code[frame.pc + 1] as usize;
    frame.op_stack.push(frame.local.get(index).unwrap().clone());
    frame.pc += 2;
}


pub fn dload(frame: &mut StackFrame) {
    let index = frame.code[frame.pc + 1] as usize;
    let v1 = frame.local.get(index).unwrap().clone();
    let v2: StackFrameValue  =  frame.local.get(index + 1).unwrap().clone();
    let mut u1:u32 = 0;
    let mut u2 : u32 = 0;

    match v1 {
        StackFrameValue::Byte(l) => u1 = l as u32,
        StackFrameValue::Char(l) => u1 = l as u32,
        StackFrameValue::Int(l) =>  u1 = l as u32,
        StackFrameValue::Short(l) => u1 = l as u32,
        StackFrameValue::U32(l) => u1 = l as u32,

        _=> panic!()
    }
    match v2 {
        StackFrameValue::Byte(l) => u2 = l as u32,
        StackFrameValue::Char(l) => u2 = l as u32,
        StackFrameValue::Int(l) =>  u2 = l as u32,
        StackFrameValue::Short(l) => u2 = l as u32,
        StackFrameValue::U32(l) => u2 = l as u32,
        _=> panic!()
    }

    let d = StackFrameValue::Double(u32_tuple_to_f64((u1,u2)));
    info!("{:?}",d);
    frame.op_stack.push(d);
    frame.pc += 2;
}

pub fn lload(frame: &mut StackFrame) {
    let index = frame.code[frame.pc + 1] as usize;
    let v1 = frame.local.get(index).unwrap().clone();
    let v2: StackFrameValue  =  frame.local.get(index + 1).unwrap().clone();
    let mut u1:u32 = 0;
    let mut u2 : u32 = 0;

    match v1 {
        StackFrameValue::Byte(l) => u1 = l as u32,
        StackFrameValue::Char(l) => u1 = l as u32,
        StackFrameValue::Int(l) =>  u1 = l as u32,
        StackFrameValue::Short(l) => u1 = l as u32,
        StackFrameValue::U32(l) => u1 = l as u32,

        _=> panic!()
    }
    match v2 {
        StackFrameValue::Byte(l) => u2 = l as u32,
        StackFrameValue::Char(l) => u2 = l as u32,
        StackFrameValue::Int(l) =>  u2 = l as u32,
        StackFrameValue::Short(l) => u2 = l as u32,
        StackFrameValue::U32(l) => u2 = l as u32,
        _=> panic!()
    }
    let d = StackFrameValue::Double(u32_tuple_to_f64((u1,u2)));
    info!("{:?}",d);
    frame.op_stack.push(d);
    frame.pc += 2;
}
pub fn iload_0(frame: &mut StackFrame) {
    frame.op_stack.push(frame.local.get(0).unwrap().clone());
    frame.pc += 1;
}


pub fn iload_1(frame: &mut StackFrame) {
    frame.op_stack.push(frame.local.get(1).unwrap().clone());
    frame.pc += 1;
}

pub fn iload_2(frame: &mut StackFrame) {
    frame.op_stack.push(frame.local.get(2).unwrap().clone());
    frame.pc += 1;
}

pub fn iload_3(frame: &mut StackFrame) {
    frame.op_stack.push(frame.local.get(3).unwrap().clone());
    frame.pc += 1;
}


pub fn fload_0(frame: &mut StackFrame) {
    frame.op_stack.push(frame.local.get(0).unwrap().clone());
    frame.pc += 1;
}


pub fn fload_1(frame: &mut StackFrame) {
    frame.op_stack.push(frame.local.get(1).unwrap().clone());
    frame.pc += 1;
}

pub fn fload_2(frame: &mut StackFrame) {
    frame.op_stack.push(frame.local.get(2).unwrap().clone());
    frame.pc += 1;
}

pub fn fload_3(frame: &mut StackFrame) {
    frame.op_stack.push(frame.local.get(3).unwrap().clone());
    frame.pc += 1;
}

pub fn lload_0(frame: &mut StackFrame) {
    let v1 = frame.local.get(0).unwrap().clone();
    let v2 = frame.local.get(1).unwrap().clone();
    match v1 {
        StackFrameValue::U32(u1) =>{
            match v2 {
                StackFrameValue::U32(u2) =>{
                    frame.op_stack.push(StackFrameValue::Long(u32_tuple_to_i64((u1,u2))));

                } ,
                _=> panic!()
            }
        }
        _=> panic!()
    }
    frame.op_stack.push(frame.local.get(0).unwrap().clone());
    frame.pc += 1;
}


pub fn lload_1(frame: &mut StackFrame) {
    let v1 = frame.local.get(1).unwrap().clone();
    let v2 = frame.local.get(2).unwrap().clone();
    match v1 {
        StackFrameValue::U32(u1) =>{
            match v2 {
                StackFrameValue::U32(u2) =>{
                    frame.op_stack.push(StackFrameValue::Long(u32_tuple_to_i64((u1,u2))));

                } ,
                _=> panic!()
            }
        }
        _=> panic!()
    }
    frame.pc += 1;
}

pub fn lload_2(frame: &mut StackFrame) {
    let v1 = frame.local.get(2).unwrap().clone();
    let v2 = frame.local.get(3).unwrap().clone();
    match v1 {
        StackFrameValue::U32(u1) =>{
            match v2 {
                StackFrameValue::U32(u2) =>{
                    frame.op_stack.push(StackFrameValue::Long(u32_tuple_to_i64((u1,u2))));

                } ,
                _=> panic!()
            }
        }
        _=> panic!()
    }
}

pub fn lload_3(frame: &mut StackFrame) {
    let v1 = frame.local.get(3).unwrap().clone();
    let v2 = frame.local.get(4).unwrap().clone();
    match v1 {
        StackFrameValue::U32(u1) =>{
            match v2 {
                StackFrameValue::U32(u2) =>{
                    frame.op_stack.push(StackFrameValue::Long(u32_tuple_to_i64((u1,u2))));
                } ,
                _=> panic!()
            }
        }
        _=> panic!()
    }
    frame.pc += 1;
}


pub fn dload_0(frame: &mut StackFrame) {
    let v1 = frame.local.get(0).unwrap().clone();
    let v2 = frame.local.get(1).unwrap().clone();
    match v1 {
        StackFrameValue::U32(u1) =>{
            match v2 {
                StackFrameValue::U32(u2) =>{
                    frame.op_stack.push(StackFrameValue::Double(u32_tuple_to_f64((u1,u2))));
                } ,
                _=> panic!()
            }
        }
        _=> panic!()
    }
    frame.op_stack.push(frame.local.get(0).unwrap().clone());
    frame.pc += 1;
}


pub fn dload_1(frame: &mut StackFrame) {
    let v1 = frame.local.get(1).unwrap().clone();
    let v2 = frame.local.get(2).unwrap().clone();
    match v1 {
        StackFrameValue::U32(u1) =>{
            match v2 {
                StackFrameValue::U32(u2) =>{
                    frame.op_stack.push(StackFrameValue::Double(u32_tuple_to_f64((u1,u2))));

                } ,
                _=> panic!()
            }
        }
        _=> panic!()
    }
}

pub fn dload_2(frame: &mut StackFrame) {
    let v1 = frame.local.get(2).unwrap().clone();
    let v2 = frame.local.get(3).unwrap().clone();
    match v1 {
        StackFrameValue::U32(u1) =>{
            match v2 {
                StackFrameValue::U32(u2) =>{
                    frame.op_stack.push(StackFrameValue::Double(u32_tuple_to_f64((u1,u2))));

                } ,
                _=> panic!()
            }
        }
        _=> panic!()
    }
}

pub fn dload_3(frame: &mut StackFrame) {
    let v1 = frame.local.get(3).unwrap().clone();
    let v2 = frame.local.get(4).unwrap().clone();
    match v1 {
        StackFrameValue::U32(u1) =>{
            match v2 {
                StackFrameValue::U32(u2) =>{
                    frame.op_stack.push(StackFrameValue::Double(u32_tuple_to_f64((u1,u2))));
                } ,
                _=> panic!()
            }
        }
        _=> panic!()
    }
}


