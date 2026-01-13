use log::info;
use log::warn;

use crate::classfile::class::ConstantPoolInfo;
use crate::common::error::Throwable;
use crate::common::reference::Reference;
use crate::common::stack_frame::StackFrame;
use crate::common::value::StackFrameValue;
use crate::runtime::heap::Heap;
use crate::runtime::metaspace::Metaspace;
use crate::utils::u8c::u8s_to_u16;



pub fn instanceof(vm_stack: &mut Vec<StackFrame>, heap: &mut Heap, metaspace: &mut Metaspace) ->Result<(),Throwable>{
    let frame_index = vm_stack.len() - 1;
    let frame = &mut vm_stack[frame_index];
    let v = frame.op_stack.pop().unwrap();
    let target_class_name = match v {
        StackFrameValue::Reference(id) => {
         let id =   heap.get_object_class_id(id as usize)?;
         Ok(metaspace.classes[id as usize].class_name.clone())
        }
        _ =>Err(Throwable::Exception(crate::common::error::Exception::NullPointer("Attempt to invoke method on null object".to_string()))),
    }?;
    let oprand = u8s_to_u16(&frame.code[(frame.pc + 1)..(frame.pc + 3)]);
    let this_class = &metaspace.classes[frame.class].clone();
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
    frame.pc += 3;
    Ok(())
}
