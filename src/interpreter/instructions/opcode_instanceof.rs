use log::info;
use log::warn;

use crate::classfile::class::ConstantPoolInfo;
use crate::common::error::Throwable;
use crate::common::reference::Reference;
use crate::common::stack_frame::StackFrame;
use crate::common::value::StackFrameValue;
use crate::runtime::runtime_data_area::get_class_name;
use crate::runtime::runtime_data_area::get_or_load_class;
use crate::runtime::runtime_data_area::get_reference;
use crate::utils::u8c::u8s_to_u16;



pub fn instanceof(frame: &mut StackFrame) ->Result<(),Throwable>{
    let v = frame.op_stack.pop().unwrap();
    let target_class_name:String = match v {
        StackFrameValue::Reference(id) => {
            let reference = get_reference(&id).unwrap();
            match reference {
                Reference::Object(object) => {
                  get_class_name(&object.class)
                }
                _ => panic!(),
            }
        }
        _ => panic!(),
    };
    let oprand = u8s_to_u16(&frame.code[(frame.pc + 1)..(frame.pc + 3)]);
    frame.pc += 3;
    let this_class = get_or_load_class(&frame.class_name).clone();
    let class_ref: &ConstantPoolInfo = &this_class.constant_pool[oprand as usize];
    match class_ref {
        ConstantPoolInfo::Class(utf8_name_index) => {
            let utfo_ref: &ConstantPoolInfo =
                &this_class.constant_pool[*utf8_name_index as usize];
            match utfo_ref {
                ConstantPoolInfo::Utf8(name) => {
                    if &target_class_name == name{
                        frame.op_stack.push(StackFrameValue::Boolean(true))
                    }else{
                        frame.op_stack.push(StackFrameValue::Boolean(false))
                    }
                }
                _ => panic!(),
            }
        }
        _ => panic!(),
    }
    Ok(())
}
