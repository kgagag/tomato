

use log::info;

use crate::{classfile::class::MethodInfo, common::stack_frame::StackFrame};

extern crate log;
extern crate env_logger;

pub fn  create_file_exclusively(method: &MethodInfo, frame: &mut StackFrame){
    let _ = method;
    info!("{:?}",frame);
}