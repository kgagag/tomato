use crate::{classfile::class::ConstantPoolInfo, common::{stack_frame::StackFrame, value::StackFrameValue}, runtime::runtime_data_area::{create_object, get_class_name, get_or_load_class}, utils::u8c::u8s_to_u16};



/**
 * 创建对象的指令
 */
pub fn _new(frame: &mut StackFrame) {
    //获取要当前类的名称
    let class_name = get_class_name(&frame.class);
    // 加载类
    let this_class = get_or_load_class(&class_name);
    //获取操作数
    let classfile_pool_index = u8s_to_u16(&frame.code[(frame.pc + 1)..(frame.pc + 3)]);
    let classfile_pool_class = &this_class.constant_pool.get(&classfile_pool_index).unwrap();
    match classfile_pool_class {
        ConstantPoolInfo::Class(name_index) => {
            let class_name_utf8 = &this_class.constant_pool.get(name_index).unwrap();
            match class_name_utf8 {
                ConstantPoolInfo::Utf8(class_name) => {
                    let target_class = get_or_load_class(class_name);
                    let obj = create_object(target_class.id);
                    //初始化属性
                    frame.op_stack.push(StackFrameValue::Reference(obj));
                    frame.pc += 3;
                }
                _ => panic!("wrong class data"),
            }
        }
        _ => panic!("wrong class data"),
    }
}