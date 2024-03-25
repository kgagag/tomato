use crate::stack_frame::StackFrame;
extern crate log;
extern crate env_logger;
use crate::class::*;
use log::{error, info, warn};

pub fn run_native(method:&MethodInfo){

    if("writeBytes" == method.method_name && "([BIIZ)" == method.descriptor && method.class_name == "java.io.FileOutputStream"){
        
    }
}