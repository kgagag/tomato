use crate::{
    object::{self, Object},
    reference::reference::Reference,
    runtime_data_area::get_reference,
    value::value::StackFrameValue,
};

pub fn dprint(msg: StackFrameValue) {
    match msg {
        StackFrameValue::Reference(id) => {
            let obj_refer = get_reference(&id);
            match obj_refer {
                Reference::Object(object) => {
                    let value = object.field.get("value").unwrap();
                    match value {
                        StackFrameValue::Reference(id) => {
                            let arr = get_reference(id);
                            match arr {
                                Reference::Array(array) => {
                                    let mut vc:Vec<char> = Vec::new();
                                    for i in 0..array.len {
                                       // print!("{:?}",);
                                       let ch = array.data.get(i as usize).unwrap();
                                       match ch {
                                           StackFrameValue::CHARACTER(c) =>{
                                                //print!("{:?}",c);
                                                vc.push(*c);
                                           }
                                           _=> panic!()
                                       }
                                     
                                    }
                                    let aa :String =  vc.into_iter().collect(); 
                                    println!("{}",aa)
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
