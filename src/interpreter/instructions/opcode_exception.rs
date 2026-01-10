
use crate::common::{error::Throwable, stack_frame::StackFrame};
extern crate log;
extern crate env_logger;

pub fn athrow(frame: &mut StackFrame) ->Result<(),Throwable>{
    frame.op_stack.push(frame.op_stack.last().unwrap().clone());
    frame.pc += 1;
    Ok(())
}