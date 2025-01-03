use log::{error, info, warn};

use crate::{classfile::class::MethodInfo, common::stack_frame::StackFrame};

use super::{
    native_array::new_array,
    native_class::{desired_assertion_status0, for_name, get_primitive_class},
    native_io::create_file_exclusively,
    native_math::{
        double_to_raw_long_bits, float_to_raw_int_bits, int_bits_to_float, long_bits_to_double,
    },
    native_net,
    native_object::{get_class, hash_code},
    native_socket_output_stream, native_stringcoding,
    native_system::{self, array_copy},
};

pub fn run_native(method: &MethodInfo, frame: &mut StackFrame) {
    if "registerNatives" != method.method_name {
        if "arraycopy" == method.method_name
            && "(Ljava/lang/Object;ILjava/lang/Object;II)V" == method.descriptor
            && method.class_name == "java/lang/System"
        {
            native_system::array_copy(method, frame);
        } else if "currentTimeMillis" == method.method_name
            && "()J" == method.descriptor
            && method.class_name == "java/lang/System"
        {
            native_system::current_time_millis(frame);
        } else if "nanoTime" == method.method_name
            && "()J" == method.descriptor
            && method.class_name == "java/lang/System"
        {
            native_system::nano_time(frame);
        } else if "desiredAssertionStatus0" == method.method_name
            && "(Ljava/lang/Class;)Z" == method.descriptor
            && method.class_name == "java/lang/Class"
        {
            desired_assertion_status0(frame);
        } else if "forName" == method.method_name
            && "(Ljava/lang/String;)Ljava/lang/Class;" == method.descriptor
            && method.class_name == "java/lang/Class"
        {
            for_name(frame);
        } else if "floatToRawIntBits" == method.method_name
            && "(F)I" == method.descriptor
            && method.class_name == "java/lang/Float"
        {
            float_to_raw_int_bits(frame);
        } else if "doubleToRawLongBits" == method.method_name
            && "(D)J" == method.descriptor
            && method.class_name == "java/lang/Double"
        {
            double_to_raw_long_bits(frame);
        } else if "getPrimitiveClass" == method.method_name
            && "(Ljava/lang/String;)Ljava/lang/Class;" == method.descriptor
            && method.class_name == "java/lang/Class"
        {
            get_primitive_class(frame);
        } else if "longBitsToDouble" == method.method_name
            && "(J)D" == method.descriptor
            && method.class_name == "java/lang/Double"
        {
            long_bits_to_double(frame);
        } else if "intBitsToFloat" == method.method_name
            && "(I)F" == method.descriptor
            && method.class_name == "java/lang/Float"
        {
            int_bits_to_float(frame);
        } else if "createFileExclusively" == method.method_name
            && "(Ljava/lang/String;)Z" == method.descriptor
            && method.class_name == "java/io/WinNTFileSystem"
        {
            create_file_exclusively(method, frame);
        } else if "hashCode" == method.method_name
            && "()I" == method.descriptor
            && method.class_name == "java/lang/Object"
        {
            hash_code(frame);
        } else if "getClass" == method.method_name
            && "()Ljava/lang/Class;" == method.descriptor
            && method.class_name == "java/lang/Object"
        {
            get_class(frame);
        } else if "newArray" == method.method_name
            && "(Ljava/lang/Class;I)Ljava/lang/Object;" == method.descriptor
            && method.class_name == "java/lang/reflect/Array"
        {
            new_array(method, frame);
        } else if "accept0" == method.method_name
            && "(Ltomato/net/Socket;)V" == method.descriptor
            && method.class_name == "tomato/net/Socket"
        {
            native_net::accept(frame);
        } else if "accept0" == method.method_name
            && "(Ltomato/net/Socket;)V" == method.descriptor
            && method.class_name == "tomato/net/Socket"
        {
            native_net::accept(frame);
        } else if "encode0" == method.method_name
            && "([CII)[B" == method.descriptor
            && method.class_name == "java/lang/StringCoding"
        {
            native_stringcoding::encode0(frame);
        } else if "decode0" == method.method_name
            && "([BII)[C" == method.descriptor
            && method.class_name == "java/lang/StringCoding"
        {
            native_stringcoding::decode0(frame);
        } else if "write0" == method.method_name
            && "([B)V" == method.descriptor
            && method.class_name == "tomato/net/SocketOutputStream"
        {
            native_socket_output_stream::write0(frame);
        } else if "close0" == method.method_name
            && "()V" == method.descriptor
            && method.class_name == "tomato/net/SocketOutputStream"
        {
            native_socket_output_stream::close0(frame);
        } else if "close0" == method.method_name
            && "()V" == method.descriptor
            && method.class_name == "tomato/net/SocketInputStream"
        {
            native_socket_output_stream::close0(frame);
        } else {
            panic!(
                "unknown native method:{},{},{}",
                method.method_name, method.descriptor, method.class_name
            );
        }
    }
}
