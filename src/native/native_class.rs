use std::f32::consts::E;

use log::info;

use crate::{
    classloader,
    common::{
        error::Throwable, param::DataType, reference::Reference, stack_frame::StackFrame,
        value::StackFrameValue,
    },
    runtime::{heap::Heap, metaspace::Metaspace},
    utils::java::{self, create_class_object},
};

pub fn desired_assertion_status0(frame: &mut StackFrame) {
    frame.op_stack.pop();
    // 暂时默认不开启
    frame.op_stack.push(StackFrameValue::Int(0))
}

pub fn for_name(
    vm_stack: &mut Vec<StackFrame>,
    heap: &mut Heap,
    metaspace: &mut Metaspace,
) -> Result<(), Throwable> {
    let frame_index = vm_stack.len() - 1;
    let sfv = vm_stack[frame_index].op_stack.pop().unwrap();
    let rust_string = java::convert_to_rust_string(sfv, vm_stack, heap, metaspace)?;
    let class_reference_id = java::create_class_object(&rust_string, vm_stack, heap, metaspace)?;
    vm_stack[frame_index]
        .op_stack
        .push(StackFrameValue::Reference(class_reference_id));
    Ok(())
}

pub fn get_primitive_class(
    vm_stack: &mut Vec<StackFrame>,
    heap: &mut Heap,
    metaspace: &mut Metaspace,
) -> Result<(), Throwable> {
    let frame_index = vm_stack.len() - 1;
    let string = {
        let object_id = match vm_stack[frame_index].op_stack.pop().unwrap() {
            StackFrameValue::Reference(id) => Ok(id),
            _ => Err(Throwable::Exception(
                crate::common::error::Exception::NullPointer("NullPointer exception".to_string()),
            )), //控制你异常
        }?;
        let class_id = heap.get_object_class_id(object_id as usize)?;
        let class = &metaspace.classes[class_id as usize];
        let mut string = String::new();
        for (_k, v) in &class.field_info {
            if v.field_name == "value" {
                let array_id = heap.get_field_ptr(object_id, v.offset);
                if array_id.is_none() {
                    return Err(Throwable::Exception(
                        crate::common::error::Exception::NullPointer(
                            "NullPointer exception".to_string(),
                        ),
                    ));
                }
                let len = heap.get_array_length(array_id.unwrap());
                for i in 0..len {
                    let (atype, value) = heap.get_array_element(array_id.unwrap(), i as usize);
                    if value.is_none() {
                        return Err(Throwable::Exception(
                            crate::common::error::Exception::NullPointer(
                                "NullPointer exception".to_string(),
                            ),
                        ));
                    }
                    if atype == 5 {
                        string.push(value.unwrap() as u8 as char);
                    }
                }
                break;
            }
        }
        string
    };

    let object_id = {
        if string == "float" {
            Ok(java::create_class_object(&String::from("java/lang/Float"), vm_stack, heap, metaspace)?)
        } else if string == "int" {
            Ok(java::create_class_object(
                &String::from("java/lang/Integer"),
                vm_stack,
                heap,
                metaspace,
            )?)
        } else if string == "long" {
            Ok(java::create_class_object(&String::from("java/lang/Long"), vm_stack, heap, metaspace)?)
        } else if string == "double" {
            Ok(java::create_class_object(&String::from("java/lang/Double"), vm_stack, heap, metaspace)?)
        } else if string == "char" {
            Ok(java::create_class_object(
                &String::from("java/lang/Character"),
                vm_stack,
                heap,
                metaspace,
            )?)
        } else if string == "short" {
            Ok(java::create_class_object(&String::from("java/lang/Short"), vm_stack, heap, metaspace)?)
        }else if  string == "boolean"  {
            Ok(java::create_class_object(&String::from("java/lang/Boolean"), vm_stack, heap, metaspace)?)
        } 
        else if  string == "byte"  {
            Ok(java::create_class_object(&String::from("java/lang/Byte"), vm_stack, heap, metaspace)?)
        } 
        else {
            Err(Throwable::Error(crate::common::error::JvmError::UnknownError("Unknown primitive class".to_string())))
        }
    }?;

    vm_stack[frame_index]
        .op_stack
        .push(StackFrameValue::Reference(object_id));
    Ok(())
}
