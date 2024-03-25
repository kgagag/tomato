use crate::runtime_data_area::pop_stack_frame;
use crate::runtime_data_area::push_frame_data;
use crate::stack_frame::StackFrame;
use crate::runtime_data_area::*;
use crate::value::value::StackFrameValue;
use crate::value::value::*;
extern crate env_logger;
extern crate log;
use log::{error, info, warn};
pub fn _return(frame: &mut StackFrame) {
    pop_stack_frame(frame.vm_stack_id);
    frame.pc += 1;
}

fn log(frame: &mut StackFrame,cursor : u64){
    let class_name = get_class_name(&frame.class);
    if(cursor == 20240325){
        info!("{} {}",class_name,"passed")
    }else if(cursor == 20240324) {
        error!("{} {}",class_name,"failed")
    }
}

pub fn ireturn(frame: &mut StackFrame) {
    let v: StackFrameValue = frame.op_stack.pop().unwrap();
    warn!("ireturn result: {:?}", &v);
    //日志埋点
    log(frame,number_u64(&v));
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
