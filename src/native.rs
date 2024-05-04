use std::f32::consts::E;
use crate::native_class::*;
use crate::native_math::*;
use crate::native_system::*;
extern crate log;
extern crate env_logger;
use crate::class::*;
use crate::StackFrame;
use log::{error, info, warn};

pub fn run_static_native(method:&MethodInfo,frame: &mut StackFrame){
    if "arraycopy" == method.method_name && "(Ljava/lang/Object;ILjava/lang/Object;II)V" == method.descriptor && method.class_name == "java/lang/System" {
        array_copy(method, frame);
    }else if "desiredAssertionStatus0" == method.method_name && "(Ljava/lang/Class;)Z" == method.descriptor && method.class_name == "java/lang/Class" {
        desired_assertion_status0(method, frame);
    }else if "floatToRawIntBits" == method.method_name && "(F)I" == method.descriptor && method.class_name == "java/lang/Float" {
        float_to_raw_int_bits(method, frame);
    }else if "doubleToRawIntBits" == method.method_name && "(D)J" == method.descriptor && method.class_name == "java/lang/Float" {
        double_to_raw_long_bits(method, frame);
    }else if "getPrimitiveClass" == method.method_name && "(Ljava/lang/String;)Ljava/lang/Class;" == method.descriptor && method.class_name == "java/lang/Class" {
        get_primitive_class(method, frame);
    }
}
