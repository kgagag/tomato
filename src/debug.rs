use log::info;

use crate::{
    object::{self, Object},
    reference::reference::Reference,
    runtime_data_area::get_reference,
    value::value::StackFrameValue,
};

pub fn dprint(msg: StackFrameValue) {
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
                                        // print!("{:?}",);
                                        let ch = array.data.get(i as usize).unwrap();
                                        match ch {
                                            StackFrameValue::CHARACTER(c) => {
                                                vc.push(*c);
                                            }
                                            StackFrameValue::Byte(c) => {
                                                vc.push((*c as u8) as char);
                                            }
                                            _ => panic!(),
                                        }
                                    }
                                    let str: String = vc.into_iter().collect();
                                    println!("{}", str)
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
        _ => panic!(),
    }
}
