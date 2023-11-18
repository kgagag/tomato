use crate::stack_frame::StackFrame;
use crate::value::value::StackFrameValue;
extern crate env_logger;
extern crate log;
use log::{error, info, warn};

pub fn i2l(frame: &mut StackFrame) {
    let v = frame.op_stack.pop().unwrap();
    match v {
        StackFrameValue::Int(i) => {
            frame.op_stack.push(StackFrameValue::Long(i as i64));
        }
        StackFrameValue::Short(i) => {
            frame.op_stack.push(StackFrameValue::Long(i as i64));
        }
        StackFrameValue::Byte(i) => {
            frame.op_stack.push(StackFrameValue::Long(i as i64));
        }
        StackFrameValue::Char(i) => {
            frame.op_stack.push(StackFrameValue::Long(i as i64));
        }
        _ => panic!(),
    }
    frame.pc += 1;
}

pub fn i2f(frame: &mut StackFrame) {
    let v = frame.op_stack.pop().unwrap();
    match v {
        StackFrameValue::Int(i) => {
            frame.op_stack.push(StackFrameValue::Float(i as f32));
        }
        StackFrameValue::Short(i) => {
            frame.op_stack.push(StackFrameValue::Float(i as f32));
        }
        StackFrameValue::Byte(i) => {
            frame.op_stack.push(StackFrameValue::Float(i as f32));
        }
        StackFrameValue::Char(i) => {
            frame.op_stack.push(StackFrameValue::Float(i as f32));
        }
        _ => panic!(),
    }
    frame.pc += 1;
}

pub fn i2d(frame: &mut StackFrame) {
    let v = frame.op_stack.pop().unwrap();
    match v {
        StackFrameValue::Int(i) => {
            frame.op_stack.push(StackFrameValue::Double(i as f64));
        }
        StackFrameValue::Short(i) => {
            frame.op_stack.push(StackFrameValue::Double(i as f64));
        }
        StackFrameValue::Byte(i) => {
            frame.op_stack.push(StackFrameValue::Double(i as f64));
        }
        StackFrameValue::Char(i) => {
            frame.op_stack.push(StackFrameValue::Double(i as f64));
        }
        _ => panic!(),
    }
    frame.pc += 1;
}

pub fn l2i(frame: &mut StackFrame) {
    let v = frame.op_stack.pop().unwrap();
    match v {
        StackFrameValue::Long(l) => {
            frame.op_stack.push(StackFrameValue::Int(l as i32));
        }
        _ => panic!(),
    }
    frame.pc += 1;
}

pub fn l2f(frame: &mut StackFrame) {
    let v = frame.op_stack.pop().unwrap();
    match v {
        StackFrameValue::Long(l) => {
            frame.op_stack.push(StackFrameValue::Float(l as f32));
        }
        _ => panic!(),
    }
    frame.pc += 1;
}

pub fn l2d(frame: &mut StackFrame) {
    let v = frame.op_stack.pop().unwrap();
    match v {
        StackFrameValue::Long(l) => {
            frame.op_stack.push(StackFrameValue::Double(l as f64));
        }
        _ => panic!(),
    }
    frame.pc += 1;
}

pub fn f2i(frame: &mut StackFrame) {
    let v = frame.op_stack.pop().unwrap();
    match v {
        StackFrameValue::Float(f) => {
            frame.op_stack.push(StackFrameValue::Int(f as i32));
        }
        _ => panic!(),
    }
    frame.pc += 1;
}

pub fn f2l(frame: &mut StackFrame) {
    let v = frame.op_stack.pop().unwrap();
    match v {
        StackFrameValue::Float(f) => {
            frame.op_stack.push(StackFrameValue::Long(f as i64));
        }
        _ => panic!(),
    }
    frame.pc += 1;
}

pub fn f2d(frame: &mut StackFrame) {
    let v = frame.op_stack.pop().unwrap();
    match v {
        StackFrameValue::Float(f) => {
            frame.op_stack.push(StackFrameValue::Double(f as f64));
        }
        _ => panic!(),
    }
    frame.pc += 1;
}

pub fn d2i(frame: &mut StackFrame) {
    let v = frame.op_stack.pop().unwrap();
    match v {
        StackFrameValue::Double(d) => {
            frame.op_stack.push(StackFrameValue::Int(d as i32));
        }
        _ => panic!(),
    }
    frame.pc += 1;
}

pub fn d2f(frame: &mut StackFrame) {
    let v = frame.op_stack.pop().unwrap();
    match v {
        StackFrameValue::Double(d) => {
            frame.op_stack.push(StackFrameValue::Float(d as f32));
        }
        _ => panic!(),
    }
    frame.pc += 1;
}

pub fn d2l(frame: &mut StackFrame) {
    let v = frame.op_stack.pop().unwrap();
    match v {
        StackFrameValue::Double(d) => {
            frame.op_stack.push(StackFrameValue::Long(d as i64));
        }
        _ => panic!(),
    }
    frame.pc += 1;
}

pub fn i2b(frame: &mut StackFrame) {
    let v = frame.op_stack.pop().unwrap();
    match v {
        StackFrameValue::Int(i) => {
            frame.op_stack.push(StackFrameValue::Byte(i as i8));
        }
        _ => panic!(),
    }
    frame.pc += 1;
}

pub fn i2c(frame: &mut StackFrame) {
    let v = frame.op_stack.pop().unwrap();
    match v {
        StackFrameValue::Int(i) => {
            frame.op_stack.push(StackFrameValue::Char(i as u16));
        }
        _ => panic!(),
    }
    frame.pc += 1;
}

pub fn i2s(frame: &mut StackFrame) {
    let v = frame.op_stack.pop().unwrap();
    match v {
        StackFrameValue::Int(i) => {
            frame.op_stack.push(StackFrameValue::Short(i as i16));
        }
        _ => panic!(),
    }
    frame.pc += 1;
}
