

use log::info;

use crate::{class::MethodInfo, stack_frame::StackFrame};
extern crate log;
extern crate env_logger;

pub fn  create_file_exclusively(method: &MethodInfo, frame: &mut StackFrame){
    info!("{:?}",frame);
}