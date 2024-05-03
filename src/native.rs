use std::f32::consts::E;

use crate::native_system::*;
extern crate log;
extern crate env_logger;
use crate::class::*;
use crate::StackFrame;
use log::{error, info, warn};

pub fn run_static_native(method:&MethodInfo,frame: &mut StackFrame){
    if "registerNatives" == method.method_name {

    }else if "writeBytes" == method.method_name && "([BIIZ)" == method.descriptor && method.class_name == "java.io.FileOutputStream" {

    }else if "arraycopy" == method.method_name && "(Ljava/lang/Object;ILjava/lang/Object;II)V" == method.descriptor && method.class_name == "java/lang/System" {
        array_copy(method, frame);
    }
}