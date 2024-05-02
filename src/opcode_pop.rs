
use crate::stack_frame::StackFrame;
use crate::value::value::StackFrameValue;
extern crate log;
extern crate env_logger;



pub fn pop(frame: &mut StackFrame) {
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

pub fn pop2(frame: &mut StackFrame) {
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
