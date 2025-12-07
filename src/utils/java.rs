use crate::{classfile::class::Class, common::{object::Object, param::DataType, reference::Reference, stack_frame::StackFrame, value::StackFrameValue}, runtime::{heap::{self, Heap}, metaspace::{self, Metaspace}}};

pub fn create_class_object(thread_id : u64,class_name: &String,heap : &mut Heap , metaspace : &mut Metaspace,vm_stack: &mut Vec<StackFrame>) -> u64 {
    let class0 = metaspace.load_class(  thread_id,&String::from("java/lang/Class"),heap, vm_stack);
    let obj_id: u64 = heap.create_object(class0.class_name);
    let id: u64 = create_string_object(class_name,thread_id,heap, metaspace,vm_stack);
    let mut referencre = heap.get_reference_mut(&obj_id).unwrap();
    match &mut *referencre {
        Reference::Object(object) => {
            object
                .field
                .insert(String::from("name"), StackFrameValue::Reference(id));
        }
        _ => panic!(),
    }
    obj_id
}

pub fn convert_to_rust_string(msg: StackFrameValue,heap : &mut Heap) ->Option<String>{
    match msg {
        StackFrameValue::Reference(id) => {
            let mut obj_refer = heap.get_reference_mut(&id).unwrap();
            match &mut *obj_refer {
                Reference::Object(object) => {
                    let value = object.field.get("value").unwrap();
                    match value {
                        StackFrameValue::Reference(id) => {
                            let mut arr = heap.get_reference_mut(id).unwrap();
                            match &mut *arr {
                                Reference::Array(array) => {
                                    let mut vc: Vec<char> = Vec::new();
                                    for i in 0..array.len {
                                        let ch = array.data.get(i as usize).unwrap();
                                        match ch {
                                            StackFrameValue::CHARACTER(c) => {
                                                vc.push(*c);
                                            }
                                            StackFrameValue::Byte(c) => {
                                                vc.push((*c as u8) as char);
                                            }
                                            _ => continue
                                        }
                                    }
                                    let str: String = vc.into_iter().collect();
                                    return Some(str);
                                }
                                _ => panic!(),
                            }
                        }
                        _ => panic!(),
                    }
                }
                _ => panic!(),
            }
        }
        StackFrameValue::Null =>{
            return None;
        }
        _=> panic!()
    }
}


pub fn create_string_object(str_value: &str,thread_id : u64,heap : &mut Heap , metaspace : &mut Metaspace,vm_stack: &mut Vec<StackFrame>) -> u64 {
    let char_array_id = {
        let chars: Vec<char> = str_value.chars().collect();
        let char_array_id: u64 = heap.create_array(chars.len() as u32, DataType::Array { element_type: (Box::new(DataType::Char)), depth: (1) });
        let mut char_array_reference = heap.get_reference_mut(&char_array_id).unwrap();
        match &mut *char_array_reference {
            Reference::Array(array) => {
                for i in 0..chars.len() {
                    array.data[i] = StackFrameValue::CHARACTER(*chars.get(i).unwrap());
                }
            }
            _ => panic!(),
        }
        char_array_id
    };
    let class_name = String::from("java/lang/String");
    let class = metaspace.load_class(  thread_id,&String::from("java/lang/Class"),heap, vm_stack);
    let obj_id: u64 = heap.create_object(class.class_name);
    let mut reference = heap.get_reference_mut(&obj_id).unwrap();
    let object: &mut Object = match &mut *reference {
        Reference::Object(obj) => obj,
        _ => panic!(),
    };
    object.field.insert(
        String::from("value"),
        StackFrameValue::Reference(char_array_id),
    );
    //metaspace::put_into_str_constant_pool(str_value.clone(), obj_id);
    metaspace.register_string_constant(class_name, obj_id);
    obj_id
}