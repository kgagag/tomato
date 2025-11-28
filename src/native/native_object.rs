use log::info;

use crate::{
    common::{
        array::array::Array,
        param::DataType,
        reference::Reference,
        stack_frame::StackFrame,
        value::StackFrameValue,
    },
    runtime::runtime_data_area::{
        get_class_name, get_constant_pool_class, get_reference, put_into_class_constant_pool,
    },
    utils::java,
};

pub fn hash_code(frame: &mut StackFrame) {
    match frame.op_stack.pop() {
        Some(StackFrameValue::Reference(id)) => {
            frame.op_stack.push(StackFrameValue::Int(id as i32));
        }
        Some(_) | None => panic!("Invalid operand stack state in hash_code"),
    }
}

pub fn get_class(frame: &mut StackFrame) {
    match frame.op_stack.pop() {
        Some(StackFrameValue::Reference(id)) => {
            let reference = match get_reference(&id) {
                Some(r) => r,
                None => panic!("Reference not found for ID: {}", id),
            };

            match reference {
                Reference::Object(object) => {
                    let class_name = get_class_name(&object.class);
                    match get_constant_pool_class(&class_name) {
                        Some(class_obj_id) => {
                            frame.op_stack.push(StackFrameValue::Reference(class_obj_id));
                        }
                        None => {
                            let obj_id = java::create_class_object(&class_name);
                            put_into_class_constant_pool(class_name.clone(), obj_id);
                            frame.op_stack.push(StackFrameValue::Reference(obj_id));
                        }
                    }
                }
                Reference::Array(array) => {
                    let array_class_name = get_array_class_name(&array);
                    let obj_id = java::create_class_object(&array_class_name);
                    frame.op_stack.push(StackFrameValue::Reference(obj_id));
                }
            }
        }
        Some(_) | None => panic!("Invalid operand stack state in get_class"),
    }
}

fn get_array_class_name(array: &Array) -> String {
    match &array.array_type {
        DataType::Array { element_type, depth } => {
            let base_char = match &**element_type {
                DataType::Byte => 'B',
                DataType::Char => 'C',
                DataType::Double => 'D',
                DataType::Float => 'F',
                DataType::Int => 'I',
                DataType::Long => 'J',
                DataType::Short => 'S',     // Fixed from 'J'
                DataType::Boolean => 'Z',   // Fixed from 'B'
                DataType::Reference(name) => return format!("{}{}", "[".repeat(*depth as usize), name),
                _ => panic!("Unsupported array element type"),
            };
            format!("{}{}", "[".repeat(*depth as usize), base_char)
        }
        _ => panic!("Expected Array type"),
    }
}