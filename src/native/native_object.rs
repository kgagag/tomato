
use log::info;

use crate::{classfile::class::MethodInfo, common::{array::array::Array, param::DataType, reference::Reference, stack_frame::StackFrame, value::StackFrameValue}, runtime::runtime_data_area::{create_class_object, get_class_name, get_constant_pool_class, get_reference, put_into_class_constant_pool}};


pub fn hash_code(method: &MethodInfo, frame: &mut StackFrame) {
    let sfv: StackFrameValue = frame.op_stack.pop().unwrap();
    match sfv {
        StackFrameValue::Reference(id) =>{
            frame.op_stack.push(StackFrameValue::Int(id as i32))
        }
        _=> panic!()
    }
 }


 pub fn get_class( frame: &mut StackFrame) {
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
                    let array_class_name = get_array_class_name(array);
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
    let _class_name:String = String::from("");
    let _prefix = String::from("");
    
    match &array.array_type {
        DataType::Array { element_type, depth } =>{
            let mut prefix = String::from("");
            
            let data_type = *element_type.clone();
            match data_type {
                DataType::Byte => {
                    for _i in 0 .. *depth{
                        prefix.push('[');
                    }
                    prefix.push('B');
                },
                DataType::Char => {
                    for _i in 0 .. *depth{
                        prefix.push('[');
                    }
                    prefix.push('C');
                }
                DataType::Double => {
                    for _i in 0 .. *depth{
                        prefix.push('[');
                    }
                    prefix.push('D');
                }
                DataType::Float => {
                    for _i in 0 .. *depth{
                        prefix.push('[');
                    }
                    prefix.push('F');
                }
                DataType::Int => {
                    for _i in 0 .. *depth{
                        prefix.push('[');
                    }
                    prefix.push('I');
                }
                DataType::Long => {
                    for _i in 0 .. *depth{
                        prefix.push('[');
                    }
                    prefix.push('J');
                }
                DataType::Reference(name) => {
                    prefix.push_str(&name);
                }
                DataType::Short => {
                    for _i in 0 .. *depth{
                        prefix.push('[');
                    }
                    prefix.push('J');
                }
                DataType::Boolean =>  {
                    for i in 0 .. *depth{
                        prefix.push('[');
                    }
                    prefix.push('B');
                }
                _=> panic!()
            }
            prefix
        }
        _=> panic!()
    }
 }