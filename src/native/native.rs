use log::{error, info, warn};

use crate::{
    classfile::class::MethodInfo, 
    common::{error::Throwable, stack_frame::StackFrame}, 
    native::native_stringcoding, 
    runtime::{heap::Heap, metaspace::Metaspace}
};

use super::{
    native_array::new_array,
    native_class::{desired_assertion_status0, for_name, get_primitive_class},
    native_io::create_file_exclusively,
    native_math::{
        double_to_raw_long_bits, float_to_raw_int_bits, int_bits_to_float, long_bits_to_double,
    },
    native_object::{get_class, hash_code},
    native_system::{self, array_copy},
    native_throwable,
};

pub fn run_native(method: &MethodInfo, vm_stack: &mut Vec<StackFrame>, heap: &mut Heap, metaspace: &mut Metaspace) -> Result<(), Throwable> {
    if "registerNatives" == method.method_name {
        return Ok(());
    }

    // 构造方法标识符，用于匹配
    let key = (method.class_name.as_str(), method.method_name.as_str(), method.descriptor.as_str());

    match key {
        ("java/lang/System", "arraycopy", "(Ljava/lang/Object;ILjava/lang/Object;II)V") => {
            native_system::array_copy(vm_stack, heap, metaspace);
            Ok(())
        }
        ("java/lang/System", "currentTimeMillis", "()J") => {
            native_system::current_time_millis(vm_stack, heap, metaspace);
            Ok(())
        }
        ("java/lang/System", "nanoTime", "()J") => {
            native_system::nano_time(vm_stack, heap, metaspace);
            Ok(())
        }
        ("java/lang/Class", "desiredAssertionStatus0", "(Ljava/lang/Class;)Z") => {
            let frame_index = vm_stack.len() - 1;
            desired_assertion_status0(&mut vm_stack[frame_index]);
            Ok(())
        }
        ("java/lang/Class", "forName", "(Ljava/lang/String;)Ljava/lang/Class;") => {
            for_name(vm_stack, heap, metaspace)
        }
        ("java/lang/Class", "getPrimitiveClass", "(Ljava/lang/String;)Ljava/lang/Class;") => {
            get_primitive_class(vm_stack, heap, metaspace)
        }
        ("java/lang/Float", "floatToRawIntBits", "(F)I") => {
            let frame_index = vm_stack.len() - 1;
            float_to_raw_int_bits(&mut vm_stack[frame_index]);
            Ok(())
        }
        ("java/lang/Double", "doubleToRawLongBits", "(D)J") => {
            let frame_index = vm_stack.len() - 1;
            double_to_raw_long_bits(&mut vm_stack[frame_index]);
            Ok(())
        }
        ("java/lang/Double", "longBitsToDouble", "(J)D") => {
            let frame_index = vm_stack.len() - 1;
            long_bits_to_double(&mut vm_stack[frame_index]);
            Ok(())
        }
        ("java/lang/Float", "intBitsToFloat", "(I)F") => {
            let frame_index = vm_stack.len() - 1;
            int_bits_to_float(&mut vm_stack[frame_index]);
            Ok(())
        }
        ("java/io/WinNTFileSystem", "createFileExclusively", "(Ljava/lang/String;)Z") => {
            let frame_index = vm_stack.len() - 1;
            create_file_exclusively(&mut vm_stack[frame_index]);
            Ok(())
        }
        ("java/lang/Object", "hashCode", "()I") => {
            let frame_index = vm_stack.len() - 1;
            hash_code(&mut vm_stack[frame_index]);
            Ok(())
        }
        ("java/lang/Object", "getClass", "()Ljava/lang/Class;") => {
            get_class(vm_stack, heap, metaspace)
        }
        ("java/lang/reflect/Array", "newArray", "(Ljava/lang/Class;I)Ljava/lang/Object;") => {
            new_array(vm_stack, heap, metaspace);
            Ok(())
        }
        ("java/lang/StringCoding", "encode0", "([CII)[B") => {
            native_stringcoding::encode0(vm_stack, heap, metaspace)
        }
        ("java/lang/StringCoding", "decode0", "([BII)[C") => {
            native_stringcoding::decode0(vm_stack, heap, metaspace)
        }
        ("java/lang/Throwable", "fillInStackTrace", "()Ljava/lang/Throwable;") => {
            native_throwable::fill_in_stack_trace(vm_stack, heap, metaspace)
        }
        _ => {
            panic!(
                "unknown native method: {}, {}, {}",
                method.method_name, method.descriptor, method.class_name
            );
        }
    }
}