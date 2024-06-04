
use log::info;

use crate::array::array::Array;
use crate::param::param::DataType;
use crate::reference::reference::Reference;
use crate::runtime_data_area::{create_class_object, get_class_name, get_constant_pool_class, get_reference, put_into_class_constant_pool};
use crate::stack_frame::StackFrame;
use crate::value::value::StackFrameValue;
extern crate env_logger;
extern crate log;
use crate::{array, class::*, object, reference};

pub fn hash_code(method: &MethodInfo, frame: &mut StackFrame) {
    let sfv: StackFrameValue = frame.op_stack.pop().unwrap();
    match sfv {
        StackFrameValue::Reference(id) =>{
            frame.op_stack.push(StackFrameValue::Int(id as i32))
        }
        _=> panic!()
    }
 }


 pub fn get_class(method: &MethodInfo, frame: &mut StackFrame) {
    let sfv: StackFrameValue = frame.op_stack.pop().unwrap();
    match sfv {
        StackFrameValue::Reference(id) =>{
            let reference = get_reference(&id).unwrap();
           // info!("{:?}",reference);
            match reference {
                Reference::Object(object) =>{
                    let class_name = get_class_name(&object.class);
                    let class_obj = get_constant_pool_class(&class_name);
                    if class_obj.is_none() {
                        let obj_id: u64 =   create_class_object(&class_name);
                        put_into_class_constant_pool(class_name.clone(), obj_id);
                        frame.op_stack.push(StackFrameValue::Reference(obj_id));
                    }else {
                        frame.op_stack.push(StackFrameValue::Reference(*class_obj.unwrap()));
                    }
                }
                Reference::Array(array) =>{
                    //info!("{:?}",array);
                    let array_class_name = get_array_class_name(&array);
                    //info!("{:?}",array_class_name);
                    let obj_id: u64 =   create_class_object(&array_class_name);
                    frame.op_stack.push(StackFrameValue::Reference(obj_id));
                }
            }
        }
        _=> panic!()
    }
 }

 fn get_array_class_name(array:&Array) ->String{
    let mut class_name:String = String::from("");
    let mut prefix = String::from("");
    
    match &array.array_type {
        DataType::Array { element_type, depth } =>{
            let mut prefix = String::from("");
            
            let data_type = *element_type.clone();
            match data_type {
                DataType::Byte => {
                    for i in 0 .. *depth{
                        prefix.push_str("[");
                    }
                    prefix.push_str("B");
                },
                DataType::Char => {
                    for i in 0 .. *depth{
                        prefix.push_str("[");
                    }
                    prefix.push_str("C");
                }
                DataType::Double => {
                    for i in 0 .. *depth{
                        prefix.push_str("[");
                    }
                    prefix.push_str("D");
                }
                DataType::Float => {
                    for i in 0 .. *depth{
                        prefix.push_str("[");
                    }
                    prefix.push_str("F");
                }
                DataType::Int => {
                    for i in 0 .. *depth{
                        prefix.push_str("[");
                    }
                    prefix.push_str("I");
                }
                DataType::Long => {
                    for i in 0 .. *depth{
                        prefix.push_str("[");
                    }
                    prefix.push_str("J");
                }
                DataType::Reference(name) => {
                    prefix.push_str(&name);
                }
                DataType::Short => {
                    for i in 0 .. *depth{
                        prefix.push_str("[");
                    }
                    prefix.push_str("J");
                }
                DataType::Boolean =>  {
                    for i in 0 .. *depth{
                        prefix.push_str("[");
                    }
                    prefix.push_str("B");
                }
                _=> panic!()
            }
            prefix
        }
        _=> panic!()
    }
 }