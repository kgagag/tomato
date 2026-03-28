use crate::{classfile::class::ConstantPoolInfo, common::{error::Throwable, stack_frame::create_stack_frame}, interpreter::instructions::op_code::op_code::do_opcode, runtime::{heap::Heap, metaspace::Metaspace}};


/**
 * 类加载完成之后执行初始化静态方法
 */
pub fn invoke(
    class_name: &String,
    method_name: String,
    heap: &mut Heap,
    metaspace: &mut Metaspace,
) -> Result<(), Throwable> {
    let class_id = *metaspace.class_map.get(class_name).unwrap();
    let class = &metaspace.classes[class_id];
    //创建VM
    //找到main方法
    for i in 0..*&class.method_info.len() {
        let method_info = &class.method_info[i];
        //let methond_index = (method_info.name_index as usize) - 1;
        let u8_vec = &class.constant_pool[method_info.name_index as usize];
        match u8_vec {
            ConstantPoolInfo::Utf8(name) => {
                if name == &method_name {
                    let mut stack_frame = create_stack_frame(method_info).unwrap();
                    let mut vm_stack = Vec::new();
                    stack_frame.vm_stack_id = 0;
                    vm_stack.push(stack_frame);
                    // 转换为可变引用（需要 unsafe）
                    let _ = do_opcode(&mut vm_stack, heap, metaspace);
                    //execute(vm_stack_id,&mut vm);
                    break;
                }
            }
            _ => {
                // return Err(Throwable::Exception(
                //     crate::common::error::j::ClassFormat {
                //         class_name: (class.class_name.clone()),
                //         message: ("class format error".to_string()),
                //     },
                // ))
                return Err(Throwable::Error(
                    crate::common::error::JvmError::ClassFormatError
                ))
            }
        }
    }
    return Ok(());
}