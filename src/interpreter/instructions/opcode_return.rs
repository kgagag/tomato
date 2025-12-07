
extern crate env_logger;
extern crate log;
use log::{error, info,  warn};

use crate::common::{stack_frame::StackFrame, value::{number_u64, StackFrameValue}};

pub fn _return(vm_stack : &mut Vec<StackFrame>) {
    let frame: &mut StackFrame = vm_stack.last_mut().unwrap();
    frame.pc += 1;
    vm_stack.pop();
}

fn debug(frame: &mut StackFrame,cursor : i32){
    if cursor == 20240325 {
        info!("{} {}",frame.class_name,"passed")
    }else if cursor == 20240324 {
        error!("{} {}",frame.class_name,"failed")
    }
}

pub fn ireturn(vm_stack : &mut Vec<StackFrame>) {
    let v: StackFrameValue = vm_stack.last_mut().unwrap().op_stack.pop().unwrap();
    //日志埋点
    debug(vm_stack.last_mut().unwrap(),number_u64(&v) as i32);
    vm_stack.pop();
    let len = vm_stack.len();
    vm_stack[len - 1].op_stack.push(v);
    //将返回值传给上一个栈帧
    //frame.pc += 1;
}

pub fn lreturn(vm_stack : &mut Vec<StackFrame>) {
    let v: StackFrameValue = vm_stack.last_mut().unwrap().op_stack.pop().unwrap();
    vm_stack.pop();
    let len = vm_stack.len();
    vm_stack[len - 1].op_stack.push(v);
    //将返回值传给上一个栈帧
    //frame.pc += 1;
}

pub fn freturn(vm_stack : &mut Vec<StackFrame>) {
    let v: StackFrameValue = vm_stack.last_mut().unwrap().op_stack.pop().unwrap();
    vm_stack.pop();
    let len = vm_stack.len();
    vm_stack[len - 1].op_stack.push(v);
}

pub fn dreturn(vm_stack : &mut Vec<StackFrame>) {
    let v: StackFrameValue = vm_stack.last_mut().unwrap().op_stack.pop().unwrap();
    vm_stack.pop();
    let len = vm_stack.len();
    vm_stack[len - 1].op_stack.push(v);
}

pub fn areturn(vm_stack : &mut Vec<StackFrame>) {
    let v: StackFrameValue = vm_stack.last_mut().unwrap().op_stack.pop().unwrap();
    vm_stack.pop();
    let len = vm_stack.len();
    vm_stack[len - 1].op_stack.push(v);
}
