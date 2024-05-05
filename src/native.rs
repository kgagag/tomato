use crate::native_class::*;
use crate::native_math::*;
use crate::native_system::*;
use std::f32::consts::E;
extern crate env_logger;
extern crate log;
use crate::class::*;
use crate::StackFrame;
use log::{error, info, warn};

pub fn run_static_native(method: &MethodInfo, frame: &mut StackFrame) {
    if "registerNatives" != method.method_name {
        if "arraycopy" == method.method_name
            && "(Ljava/lang/Object;ILjava/lang/Object;II)V" == method.descriptor
            && method.class_name == "java/lang/System"
        {
            array_copy(method, frame);
        } else if "desiredAssertionStatus0" == method.method_name
            && "(Ljava/lang/Class;)Z" == method.descriptor
            && method.class_name == "java/lang/Class"
        {
            desired_assertion_status0(method, frame);
        } else if "floatToRawIntBits" == method.method_name
            && "(F)I" == method.descriptor
            && method.class_name == "java/lang/Float"
        {
            float_to_raw_int_bits(method, frame);
        } else if "doubleToRawLongBits" == method.method_name
            && "(D)J" == method.descriptor
            && method.class_name == "java/lang/Double"
        {
            double_to_raw_long_bits(method, frame);
        } else if "getPrimitiveClass" == method.method_name
            && "(Ljava/lang/String;)Ljava/lang/Class;" == method.descriptor
            && method.class_name == "java/lang/Class"
        {
            get_primitive_class(frame);
        } else if "longBitsToDouble" == method.method_name
            && "(J)D" == method.descriptor
            && method.class_name == "java/lang/Double"
        {
            long_bits_to_double(method, frame);
        } else if "intBitsToFloat" == method.method_name
            && "(I)F" == method.descriptor
            && method.class_name == "java/lang/Float"
        {
            int_bits_to_float(method, frame);
        } else {
            panic!("unknown native method");
        }
    }
}
