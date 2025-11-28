
extern crate env_logger;
extern crate log;
use log::{error, info,  warn};

use crate::{common::{stack_frame::StackFrame, value::{number_u64, StackFrameValue}}, runtime::runtime_data_area::{get_class_name}};

pub fn _return(vm_stack: &mut Vec<StackFrame>) {
    vm_stack.pop();
}

fn debug(frame : &mut StackFrame ,cursor : i32){
    let class_name = get_class_name(&frame.class);
    if cursor == 20240325 {
        info!("{} {}",class_name,"passed")
    }else if cursor == 20240324 {
        error!("{} {}",class_name,"failed")
    }
}

pub fn ireturn(vm_stack: &mut Vec<StackFrame>) {
    let v: StackFrameValue = vm_stack.last_mut().unwrap().op_stack.pop().unwrap();
    vm_stack.pop();
    let len = vm_stack.len();
    vm_stack[len - 1].op_stack.push(v);
}

pub fn lreturn(vm_stack: &mut Vec<StackFrame>) {
    let v: StackFrameValue = vm_stack.last_mut().unwrap().op_stack.pop().unwrap();
    vm_stack.pop();
    let len = vm_stack.len();
    vm_stack[len - 1].op_stack.push(v);
}

pub fn freturn(vm_stack: &mut Vec<StackFrame>) {
    let v: StackFrameValue = vm_stack.last_mut().unwrap().op_stack.pop().unwrap();
    vm_stack.pop();
    let len = vm_stack.len();
    vm_stack[len - 1].op_stack.push(v);
}

pub fn dreturn(vm_stack: &mut Vec<StackFrame>) {
   let v: StackFrameValue = vm_stack.last_mut().unwrap().op_stack.pop().unwrap();
    vm_stack.pop();
    let len = vm_stack.len();
    vm_stack[len - 1].op_stack.push(v);
}

pub fn areturn(vm_stack: &mut Vec<StackFrame>) {
    let v: StackFrameValue = vm_stack.last_mut().unwrap().op_stack.pop().unwrap();
    vm_stack.pop();
    let len = vm_stack.len();
    vm_stack[len - 1].op_stack.push(v);
}
