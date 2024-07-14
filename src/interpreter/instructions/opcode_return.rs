
extern crate env_logger;
extern crate log;
use log::{error, info,  warn};

use crate::{common::{stack_frame::StackFrame, value::{number_u64, StackFrameValue}}, runtime::runtime_data_area::{get_class_name, pop_stack_frame, push_frame_data}};

pub fn _return(frame: &mut StackFrame) {
    pop_stack_frame(frame.vm_stack_id);
    frame.pc += 1;
}

fn debug(frame: &mut StackFrame,cursor : i32){
    let class_name = get_class_name(&frame.class);
    if cursor == 20240325 {
        info!("{} {}",class_name,"passed")
    }else if(cursor == 20240324) {
        error!("{} {}",class_name,"failed")
    }
}

pub fn ireturn(frame: &mut StackFrame) {
    let v: StackFrameValue = frame.op_stack.pop().unwrap();
    //日志埋点
    debug(frame,number_u64(&v) as i32);
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
     //warn!!("ireturn result: {:?}", &v);
    pop_stack_frame(frame.vm_stack_id);
    push_frame_data(frame.vm_stack_id, v);
    //将返回值传给上一个栈帧
    frame.pc += 1;
}

pub fn dreturn(frame: &mut StackFrame) {
    let v: StackFrameValue = frame.op_stack.pop().unwrap();
     //warn!!("ireturn result: {:?}", &v);
    pop_stack_frame(frame.vm_stack_id);
    push_frame_data(frame.vm_stack_id, v);
    //将返回值传给上一个栈帧
    frame.pc += 1;
}

pub fn areturn(frame: &mut StackFrame) {
    let v: StackFrameValue = frame.op_stack.pop().unwrap();
    pop_stack_frame(frame.vm_stack_id);
    push_frame_data(frame.vm_stack_id, v);
    //将返回值传给上一个栈帧
    frame.pc += 1;
}
