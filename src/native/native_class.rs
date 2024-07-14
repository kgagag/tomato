use byteorder::LE;
use log::info;

use crate::{classfile::class::MethodInfo, common::{reference::Reference, stack_frame::StackFrame, value::StackFrameValue}, runtime::runtime_data_area::{create_class_object, get_reference}};


pub fn desired_assertion_status0(method: &MethodInfo, frame: &mut StackFrame) {
    frame.op_stack.pop();
    // 暂时默认不开启
    frame.op_stack.push(StackFrameValue::Int(0))
}

pub fn get_primitive_class(frame: &mut StackFrame) {
    let param = match frame.op_stack.pop().unwrap() {
        StackFrameValue::Reference(id) => get_reference(&id).unwrap(),
        _ => panic!(),
    };

    match param {
        Reference::Object(object) => {
            let sfv = object.field.get("value").unwrap();
            match sfv {
                StackFrameValue::Reference(arr_id) => {
                    let arr = get_reference(arr_id).unwrap();
                    match arr {
                        Reference::Array(array) => {
                            let mut v: Vec<char> = Vec::new();
                            for i in 0..array.data.len() {
                                match array.data.get(i).unwrap() {
                                    StackFrameValue::CHARACTER(c) => {
                                        v.push(*c);
                                    }
                                    _ => panic!(),
                                }
                            }
                            let class_object: u64;
                            if v == vec!['f', 'l', 'o', 'a', 't'] {
                                class_object =
                                    create_class_object(&String::from("java/lang/Float"));
                            } else if v == vec!['i', 'n', 't'] {
                                class_object =
                                    create_class_object(&String::from("java/lang/Integer"));
                            } else if v == vec!['l', 'o', 'n', 'g'] {
                                class_object = create_class_object(&String::from("java/lang/Long"));
                            } else if v == vec!['s', 'h', 'o', 'r', 't'] {
                                class_object =
                                    create_class_object(&String::from("java/lang/Short"));
                            } else if v == vec!['d', 'o', 'u', 'b', 'l', 'e'] {
                                class_object =
                                    create_class_object(&String::from("java/lang/Double"));
                            } else if v == vec!['c', 'h', 'a', 'r'] {
                                class_object =
                                    create_class_object(&String::from("java/lang/Character"));
                            } else {
                                panic!()
                            }
                            frame
                                .op_stack
                                .push(StackFrameValue::Reference(class_object))
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

