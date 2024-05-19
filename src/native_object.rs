
use crate::reference::reference::Reference;
use crate::runtime_data_area::{create_class_object, get_class_name, get_constant_pool_class, get_reference, put_into_class_constant_pool};
use crate::stack_frame::StackFrame;
use crate::value::value::StackFrameValue;
extern crate env_logger;
extern crate log;
use crate::{class::*, object, reference};
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
                _=> panic!()
            }
        }
        _=> panic!()
    }
 }