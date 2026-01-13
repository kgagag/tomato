use log::info;

use crate::{
    common::{
        array::array::Array, error::Throwable, param::DataType, reference::Reference,
        stack_frame::StackFrame, value::StackFrameValue,
    },
    runtime::{
        heap::{self, Heap},
        metaspace::Metaspace,
    },
    utils::java,
};

pub fn hash_code(frame: &mut StackFrame) {
    let sfv: StackFrameValue = frame.op_stack.pop().unwrap();
    match sfv {
        StackFrameValue::Reference(id) => frame.op_stack.push(StackFrameValue::Int(id as i32)),
        _ => panic!(),
    }
}

pub fn get_class(
    vm_stack: &mut Vec<StackFrame>,
    heap: &mut Heap,
    metaspace: &mut Metaspace,
) -> Result<(), Throwable> {
    let frame_index = vm_stack.len() - 1;
    let sfv: StackFrameValue = vm_stack[frame_index].op_stack.pop().unwrap();
    match sfv {
        StackFrameValue::Reference(id) => {
            // let reference = get_reference(&id).unwrap();
            // info!("{:?}",reference);
            let class_id = heap.get_object_class_id(id as usize)?;
            if !heap.is_array(id as usize) {
                let class_obj = heap.get_constant_pool_class(&class_id);
                if class_obj.is_some() {
                    vm_stack[frame_index]
                        .op_stack
                        .push(StackFrameValue::Reference(*class_obj.unwrap()));
                } else {
                    let class_name = &metaspace.classes[class_id as usize].class_name.clone();
                    let class_obj_id =
                        java::create_class_object(class_name, vm_stack, heap, metaspace)?;
                    heap.put_into_class_constant_pool(class_id, class_obj_id);
                    vm_stack[frame_index]
                        .op_stack
                        .push(StackFrameValue::Reference(class_obj_id))
                }
            } else {
                let class_obj_id =
                    java::create_class_object(&"ARRAY".to_string(), vm_stack, heap, metaspace)?;
                vm_stack[frame_index]
                    .op_stack
                    .push(StackFrameValue::Reference(class_obj_id))
            }
            Ok(())
        }
        _ => panic!(),
    }
}

fn get_array_class_name(array: &Array) -> String {
    let _class_name: String = String::from("");
    let _prefix = String::from("");

    match &array.array_type {
        DataType::Array {
            element_type,
            depth,
        } => {
            let mut prefix = String::from("");

            let data_type = *element_type.clone();
            match data_type {
                DataType::Byte => {
                    for _i in 0..*depth {
                        prefix.push('[');
                    }
                    prefix.push('B');
                }
                DataType::Char => {
                    for _i in 0..*depth {
                        prefix.push('[');
                    }
                    prefix.push('C');
                }
                DataType::Double => {
                    for _i in 0..*depth {
                        prefix.push('[');
                    }
                    prefix.push('D');
                }
                DataType::Float => {
                    for _i in 0..*depth {
                        prefix.push('[');
                    }
                    prefix.push('F');
                }
                DataType::Int => {
                    for _i in 0..*depth {
                        prefix.push('[');
                    }
                    prefix.push('I');
                }
                DataType::Long => {
                    for _i in 0..*depth {
                        prefix.push('[');
                    }
                    prefix.push('J');
                }
                DataType::Reference(name) => {
                    prefix.push_str(&name);
                }
                DataType::Short => {
                    for _i in 0..*depth {
                        prefix.push('[');
                    }
                    prefix.push('J');
                }
                DataType::Boolean => {
                    for _i in 0..*depth {
                        prefix.push('[');
                    }
                    prefix.push('B');
                }
                _ => panic!(),
            }
            prefix
        }
        _ => panic!(),
    }
}
