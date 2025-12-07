
use crate::common::stack_frame::StackFrame;
extern crate log;
extern crate env_logger;


pub fn nop(vm_stack: &mut Vec<StackFrame>) {
    vm_stack.last_mut().unwrap().pc += 1;
}