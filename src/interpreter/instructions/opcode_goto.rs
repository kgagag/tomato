use crate::{common::{error::Throwable, stack_frame::StackFrame}, utils::u8c::u8s_to_u16};

extern crate log;
extern crate env_logger;

pub fn goto(frame: &mut StackFrame) ->Result<(),Throwable>{
    let branch_offset = u8s_to_u16(&frame.code[frame.pc + 1.. frame.pc + 3]) as i16;
    frame.pc = (frame.pc as i32 + branch_offset as i32) as usize;
    Ok(())
}

pub fn jsr(frame: &mut StackFrame) ->Result<(),Throwable>{
    let _ = frame;
    Ok(())
    
}

pub fn ret(frame: &mut StackFrame) ->Result<(),Throwable>{
   Ok(())
}

