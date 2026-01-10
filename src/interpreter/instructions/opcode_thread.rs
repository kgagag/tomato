use crate::common::{error::Throwable, stack_frame::StackFrame};


extern crate log;
extern crate env_logger;


pub fn monitorenter(frame: &mut StackFrame) ->Result<(), Throwable> {
    frame.pc += 1;
    //log::info!("monitorenter");
    Ok(())
}


pub fn monitorexit(frame: &mut StackFrame) ->Result<(), Throwable>{
   // frame.pc += 1;
    Ok(())
}