use std::{cell::RefCell, sync::Arc};

use log::info;

use crate::{
    classfile::class::ConstantPoolInfo,
    classloader::class_loader,
    common::{error::Throwable, stack_frame::StackFrame, value::StackFrameValue},
    runtime::{
        heap::{self, Heap},
        metaspace::Metaspace,
    },
    utils::u8c::u8s_to_u16,
};

/**
 * 创建对象的指令
 */
pub fn _new(
    vm_stack: &mut Vec<StackFrame>,
    heap: &mut Heap,
    metaspace: &mut Metaspace,
) -> Result<(), Throwable> {
    let frame_index = vm_stack.len() - 1;
    let mut target_class_name = {
        let this_class = &metaspace.classes[vm_stack[frame_index].class];
        let classfile_pool_index = u8s_to_u16(
            &vm_stack[frame_index].code
                [(vm_stack[frame_index].pc + 1)..(vm_stack[frame_index].pc + 3)],
        );
        let classfile_pool_class = &this_class.constant_pool[classfile_pool_index as usize];
        match classfile_pool_class {
            ConstantPoolInfo::Class(name_index) => {
                let class_name_utf8 = &this_class.constant_pool[*name_index as usize];
                match class_name_utf8 {
                    ConstantPoolInfo::Utf8(class_name) => class_name.clone(),
                    _ => {
                        return Err(Throwable::Error(
                            crate::common::error::JvmError::InternalError {
                                message: "Internal error".to_string(),
                                line_number: line!(),
                                file_name: file!().to_string(),
                            },
                        ));
                    }
                }
            }
            _ => {
                return Err(Throwable::Error(
                    crate::common::error::JvmError::ClassFormatError {
                         class_name: this_class.class_name.clone(), message: "ClassFormatError".to_string() 
                        }
                ));
            }
        }
    };
    let target_class: &crate::classfile::class::Class =
        { class_loader::find_class(&mut target_class_name, vm_stack, heap, metaspace)? };
    let obj: usize = heap.create_object(target_class)?;
    //初始化属性
    vm_stack[frame_index]
        .op_stack
        .push(StackFrameValue::Reference(obj as u32));

    vm_stack[frame_index].pc += 3;
    Ok(())
}
