use crate::common::{stack_frame::StackFrame, value::StackFrameValue};

extern crate log;
extern crate env_logger;



pub fn pop(vm_stack: &mut Vec<StackFrame>) {
    let frame = vm_stack.last_mut().unwrap();
    let v = frame.op_stack.last().unwrap();
    match v {
        StackFrameValue::Double(_d) =>{
            panic!()
        } 
        StackFrameValue::Long(l) =>{
            panic!()
        } 
        _=> {
            frame.op_stack.pop();
        }
    }
    frame.pc += 1;
}

pub fn pop2(vm_stack: &mut Vec<StackFrame>) {
    let frame = vm_stack.last_mut().unwrap();
    let v = frame.op_stack.last().unwrap();
    match v {
        StackFrameValue::Double(_d) =>{
            frame.op_stack.pop();
        } 
        StackFrameValue::Long(_l) =>{
            frame.op_stack.pop();
        } 
        _=> panic!()
    }
    frame.pc += 1;
}
