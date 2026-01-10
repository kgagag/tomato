
use crate::common::{error::Throwable, stack_frame::StackFrame};
extern crate log;
extern crate env_logger;


pub fn nop(frame: &mut StackFrame) ->Result<(),Throwable> {
    frame.pc += 1;
    Ok(())
}