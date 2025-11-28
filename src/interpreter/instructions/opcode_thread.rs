use crate::common::stack_frame::StackFrame;


extern crate log;
extern crate env_logger;


pub fn monitorenter(vm_stack: &mut Vec<StackFrame>) {
    let frame = vm_stack.last_mut().unwrap();
    frame.pc += 1;
}


pub fn monitorexit(vm_stack: &mut Vec<StackFrame>) {
    let frame = vm_stack.last_mut().unwrap();
    frame.pc += 1;
}