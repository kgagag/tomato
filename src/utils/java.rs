use crate::{classfile::class::Class, common::{object::Object, param::DataType, reference::Reference, value::StackFrameValue}, runtime::runtime_data_area::{self, create_array, create_object, get_or_load_class, get_reference}};

pub fn create_class_object(class_name: &String) -> u64 {
    let class0 = get_or_load_class(&String::from("java/lang/Class"));
    let obj_id: u64 = create_object(class0.id as usize);
    let id = create_string_object(class_name.clone());
    let referencre: &mut Reference = get_reference(&obj_id).unwrap();
    match referencre {
        Reference::Object(object) => {
            object
                .field
                .insert(String::from("name"), StackFrameValue::Reference(id));
        }
        _ => panic!(),
    }
    obj_id
}

pub fn convert_to_rust_string(msg: StackFrameValue) ->Option<String>{
    match msg {
        StackFrameValue::Reference(id) => {
            let obj_refer = get_reference(&id).unwrap();
            match obj_refer {
                Reference::Object(object) => {
                    let value = object.field.get("value").unwrap();
                    match value {
                        StackFrameValue::Reference(id) => {
                            let arr = get_reference(id).unwrap();
                            match arr {
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


pub fn create_string_object(str_value: String) -> u64 {
    let char_array_id = {
        let chars: Vec<char> = str_value.chars().collect();
        let char_array_id: u64 = create_array(chars.len() as u32, DataType::Array { element_type: (Box::new(DataType::Char)), depth: (1) });
        let char_array_reference = get_reference(&char_array_id).unwrap();
        match char_array_reference {
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
    let class: &mut Class = runtime_data_area::get_or_load_class(&class_name);
    let obj_id: u64 = runtime_data_area::create_object(class.id);
    let reference = runtime_data_area::get_reference(&obj_id).unwrap();
    let object: &mut Object = match reference {
        Reference::Object(obj) => obj,
        _ => panic!(),
    };

    object.field.insert(
        String::from("value"),
        StackFrameValue::Reference(char_array_id),
    );
    runtime_data_area::put_into_str_constant_pool(str_value.clone(), obj_id);
    obj_id
}