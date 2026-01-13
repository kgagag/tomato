use log::info;

use crate::{common::{error::Throwable, stack_frame::StackFrame, value::StackFrameValue}, interpreter::instructions::opcode_math::ineg, runtime::{heap::Heap, metaspace::Metaspace}};

use super::java;



pub fn dprint(msg: StackFrameValue,vm_stack:&mut Vec<StackFrame>,heap:&mut Heap,metaspace:&mut Metaspace) -> Result<(), Throwable> {
   let rust_string =  java::convert_to_rust_string(msg,vm_stack,heap,metaspace)?;
   println!("{}", rust_string);
   Ok(())
}
