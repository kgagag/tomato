
extern crate env_logger;
extern crate log;
use log::{error, info,  warn};

use crate::{common::{stack_frame::StackFrame, value::{self, StackFrameValue, as_i32, number_u64}}, runtime::runtime_data_area::{get_class_name, pop_stack_frame, push_frame_data}};

pub fn _return(vm_stack:&mut Vec<StackFrame>) {
    let frame_index = vm_stack.len() - 1;
    vm_stack[frame_index].pc += 1;
    vm_stack.pop();
}

fn debug(frame: &mut StackFrame,cursor : i32){
    if cursor == 20240325 {
        info!("{} {}",frame.class_name,"passed")
    }else if cursor == 20240324 {
        error!("{} {}",frame.class_name,"failed")
    }
}

pub fn ireturn(vm_stack:&mut Vec<StackFrame>) {
    let frame_index = vm_stack.len() - 1;
    vm_stack[frame_index].pc += 1;
    let v: StackFrameValue = vm_stack[frame_index].op_stack.pop().unwrap();
    //日志埋点
    debug(&mut vm_stack[frame_index],value::as_i32(&v));
    vm_stack.pop();
    let frame_index: usize = vm_stack.len() - 1;
    vm_stack[frame_index].op_stack.push(v);
}

pub fn lreturn(vm_stack:&mut Vec<StackFrame>) {
    let frame_index = vm_stack.len() - 1;
    vm_stack[frame_index].pc += 1;
    let v: StackFrameValue = vm_stack[frame_index].op_stack.pop().unwrap();
    vm_stack.pop();
    let frame_index: usize = vm_stack.len() - 1;
    vm_stack[frame_index].op_stack.push(v);
}

pub fn freturn(vm_stack:&mut Vec<StackFrame>) {
    let frame_index = vm_stack.len() - 1;
    vm_stack[frame_index].pc += 1;
    let v: StackFrameValue = vm_stack[frame_index].op_stack.pop().unwrap();
    vm_stack.pop();
    let frame_index: usize = vm_stack.len() - 1;
    vm_stack[frame_index].op_stack.push(v);
}

pub fn dreturn(vm_stack:&mut Vec<StackFrame>) {
    let frame_index = vm_stack.len() - 1;
    vm_stack[frame_index].pc += 1;
    let v: StackFrameValue = vm_stack[frame_index].op_stack.pop().unwrap();
    vm_stack.pop();
    let frame_index: usize = vm_stack.len() - 1;
    vm_stack[frame_index].op_stack.push(v);
}

pub fn areturn(vm_stack:&mut Vec<StackFrame>) {
    let frame_index = vm_stack.len() - 1;
    vm_stack[frame_index].pc += 1;
    let v: StackFrameValue = vm_stack[frame_index].op_stack.pop().unwrap();
    vm_stack.pop();
    let frame_index: usize = vm_stack.len() - 1;
    vm_stack[frame_index].op_stack.push(v);
}
