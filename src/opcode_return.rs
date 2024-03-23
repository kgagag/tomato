use crate::runtime_data_area::pop_stack_frame;
use crate::runtime_data_area::push_frame_data;
use crate::stack_frame::StackFrame;
use crate::value::value::StackFrameValue;
extern crate env_logger;
extern crate log;
use log::{error, info, warn};
pub fn _return(frame: &mut StackFrame) {
    pop_stack_frame(frame.vm_stack_id);
    frame.pc += 1;
}

pub fn ireturn(frame: &mut StackFrame) {
    let v: StackFrameValue = frame.op_stack.pop().unwrap();
    warn!("ireturn result: {:?}", &v);
    pop_stack_frame(frame.vm_stack_id);
    push_frame_data(frame.vm_stack_id, v);
    //将返回值传给上一个栈帧
    frame.pc += 1;
}

pub fn lreturn(frame: &mut StackFrame) {
    let v: StackFrameValue = frame.op_stack.pop().unwrap();
    warn!("ireturn result: {:?}", &v);
    pop_stack_frame(frame.vm_stack_id);
    push_frame_data(frame.vm_stack_id, v);
    //将返回值传给上一个栈帧
    frame.pc += 1;
}

pub fn freturn(frame: &mut StackFrame) {
    let v: StackFrameValue = frame.op_stack.pop().unwrap();
    warn!("ireturn result: {:?}", &v);
    pop_stack_frame(frame.vm_stack_id);
    push_frame_data(frame.vm_stack_id, v);
    //将返回值传给上一个栈帧
    frame.pc += 1;
}

pub fn dreturn(frame: &mut StackFrame) {
    let v: StackFrameValue = frame.op_stack.pop().unwrap();
    warn!("ireturn result: {:?}", &v);
    pop_stack_frame(frame.vm_stack_id);
    push_frame_data(frame.vm_stack_id, v);
    //将返回值传给上一个栈帧
    frame.pc += 1;
}

pub fn areturn(frame: &mut StackFrame) {
    let v: StackFrameValue = frame.op_stack.pop().unwrap();
    warn!("ireturn result: {:?}", &v);
    pop_stack_frame(frame.vm_stack_id);
    push_frame_data(frame.vm_stack_id, v);
    //将返回值传给上一个栈帧
    frame.pc += 1;
}
