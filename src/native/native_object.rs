use std::f32::consts::E;

use log::info;

use crate::{
    common::{
        array::array::Array, error::{JvmError, Throwable}, param::DataType, reference::Reference,
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
            if !heap.is_array(id as usize) {
                let class_id: u32 = heap.get_object_class_id(id as usize)?;
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
                let (class_id, atype, dimension) = heap.get_array_info(id);
                let class_name = {
                    if class_id.is_some(){
                        get_basic_array_class_name(Some( metaspace.classes[class_id.unwrap() as usize].class_name.clone()),atype, dimension)?
                    }else{
                        get_basic_array_class_name(None,atype, dimension)?
                    }
                };
                let class_obj_id =
                    java::create_class_object(&class_name, vm_stack, heap, metaspace)?;
                 vm_stack[frame_index]
                    .op_stack
                    .push(StackFrameValue::Reference(class_obj_id))
            }
            Ok(())
        }
        _ => panic!(),
    }
}

fn get_basic_array_class_name(string:Option<String>, atype: u8, dimension: u8) -> Result<String,Throwable> {
    let mut class_name: String = String::from("");
    for _i in 0..dimension {
        class_name.push('[');
    }
    if atype == 4 {
        // array_type = DataType::Boolean;
        class_name.push('Z');
    } else if atype == 5 {
        // array_type = DataType::Char;
        class_name.push('C');
    } else if atype == 6 {
        // array_type = DataType::Float;
        class_name.push('F');
    } else if atype == 7 {
        //  array_type = DataType::Double;
        class_name.push('D');
    } else if atype == 8 {
        //  array_type = DataType::Byte;
        class_name.push('B');
    } else if atype == 9 {
        // array_type = DataType::Short;
        class_name.push('S');
    } else if atype == 10 {
        // array_type = DataType::Int;
        class_name.push('I');
    } else if atype == 11 {
        // array_type = DataType::Long;
        class_name.push('J');
    }else if atype == 12 {
        // array_type = DataType::Object;
        class_name.push('L');
        class_name.push_str(string.unwrap().as_str());
        class_name.push(';');
    }
    else {
        return Err(Throwable::Error(JvmError::InternalError("Internal error".to_string())));
    }
    Ok(class_name)
}
