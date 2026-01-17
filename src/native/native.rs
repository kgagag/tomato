use log::{error, info, warn};

use crate::{classfile::class::MethodInfo, common::{error::Throwable, stack_frame::StackFrame}, native::native_stringcoding, runtime::{heap::Heap, metaspace::Metaspace}};

use super::{
    native_array::new_array,
    native_class::{desired_assertion_status0, for_name, get_primitive_class},
    native_io::create_file_exclusively,
    native_math::{
        double_to_raw_long_bits, float_to_raw_int_bits, int_bits_to_float, long_bits_to_double,
    },
    native_object::{get_class, hash_code},
    native_system::{self, array_copy},
};

pub fn run_native(method: &MethodInfo,vm_stack: &mut Vec<StackFrame>, heap: &mut Heap, metaspace: &mut Metaspace) ->Result<(),Throwable> {
    
    let frame_index = vm_stack.len() - 1;
    
    if "registerNatives" != method.method_name {
        if "arraycopy" == method.method_name
            && "(Ljava/lang/Object;ILjava/lang/Object;II)V" == method.descriptor
            && method.class_name == "java/lang/System"
        {
            native_system::array_copy(method, vm_stack,heap,metaspace);
        } else if "currentTimeMillis" == method.method_name
            && "()J" == method.descriptor
            && method.class_name == "java/lang/System"
        {
            native_system::current_time_millis(vm_stack,heap,metaspace);
        } else if "nanoTime" == method.method_name
            && "()J" == method.descriptor
            && method.class_name == "java/lang/System"
        {
            native_system::nano_time(vm_stack,heap,metaspace);
        } else if "desiredAssertionStatus0" == method.method_name
            && "(Ljava/lang/Class;)Z" == method.descriptor
            && method.class_name == "java/lang/Class"
        {
            desired_assertion_status0(&mut vm_stack[frame_index]);
        } else if "forName" == method.method_name
            && "(Ljava/lang/String;)Ljava/lang/Class;" == method.descriptor
            && method.class_name == "java/lang/Class"
        {
            for_name(vm_stack,heap,metaspace);
        } else if "floatToRawIntBits" == method.method_name
            && "(F)I" == method.descriptor
            && method.class_name == "java/lang/Float"
        {
            float_to_raw_int_bits(&mut vm_stack[frame_index]);
        } else if "doubleToRawLongBits" == method.method_name
            && "(D)J" == method.descriptor
            && method.class_name == "java/lang/Double"
        {
            double_to_raw_long_bits(&mut vm_stack[frame_index]);
        } else if "getPrimitiveClass" == method.method_name
            && "(Ljava/lang/String;)Ljava/lang/Class;" == method.descriptor
            && method.class_name == "java/lang/Class"
        {
            get_primitive_class(vm_stack,heap,metaspace);
        } else if "longBitsToDouble" == method.method_name
            && "(J)D" == method.descriptor
            && method.class_name == "java/lang/Double"
        {
            long_bits_to_double(&mut vm_stack[frame_index]);
        } else if "intBitsToFloat" == method.method_name
            && "(I)F" == method.descriptor
            && method.class_name == "java/lang/Float"
        {
            int_bits_to_float(&mut vm_stack[frame_index]);
        } else if "createFileExclusively" == method.method_name
            && "(Ljava/lang/String;)Z" == method.descriptor
            && method.class_name == "java/io/WinNTFileSystem"
        {
            create_file_exclusively(method, &mut vm_stack[frame_index]);
        } else if "hashCode" == method.method_name
            && "()I" == method.descriptor
            && method.class_name == "java/lang/Object"
        {
            hash_code(&mut vm_stack[frame_index]);
        } else if "getClass" == method.method_name
            && "()Ljava/lang/Class;" == method.descriptor
            && method.class_name == "java/lang/Object"
        {
            get_class(vm_stack,heap,metaspace);
        } else if "newArray" == method.method_name
            && "(Ljava/lang/Class;I)Ljava/lang/Object;" == method.descriptor
            && method.class_name == "java/lang/reflect/Array"
        {
            new_array(method, vm_stack,heap,metaspace);
        }
        
         else if "accept0" == method.method_name
            && "(Ltomato/net/Socket;)V" == method.descriptor
            && method.class_name == "tomato/net/Socket"
        {
           // native_net::accept(vm_stack,heap,metaspace);
        } else if "accept0" == method.method_name
            && "(Ltomato/net/Socket;)V" == method.descriptor
            && method.class_name == "tomato/net/Socket"
        {
           // native_net::accept(vm_stack,heap,metaspace);
        } else if "encode0" == method.method_name
            && "([CII)[B" == method.descriptor
            && method.class_name == "java/lang/StringCoding"
        {
            native_stringcoding::encode0(vm_stack,heap,metaspace);
        } else if "decode0" == method.method_name
            && "([BII)[C" == method.descriptor
            && method.class_name == "java/lang/StringCoding"
        {
            native_stringcoding::decode0(vm_stack,heap,metaspace);
        } else if "write0" == method.method_name
            && "([B)V" == method.descriptor
            && method.class_name == "tomato/net/SocketOutputStream"
        {
           // native_socket_output_stream::write0(vm_stack,heap,metaspace);
        } else if "close0" == method.method_name
            && "()V" == method.descriptor
            && method.class_name == "tomato/net/SocketOutputStream"
        {
           // native_socket_output_stream::close0(vm_stack,heap,metaspace);
        } else if "close0" == method.method_name
            && "()V" == method.descriptor
            && method.class_name == "tomato/net/SocketInputStream"
        {
           // native_socket_output_stream::close0(vm_stack,heap,metaspace);
        }
        
         else {
            panic!(
                "unknown native method:{},{},{}",
                method.method_name, method.descriptor, method.class_name
            );
        }
    }
    Ok(())
}
