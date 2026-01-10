use std::array;
use std::f32::consts::E;
use std::thread::panicking;

use crate::common::error::Throwable;
use crate::common::stack_frame::StackFrame;



pub fn checkcast(frame: &mut StackFrame) ->Result<(),Throwable> {
    // let sfv =frame.op_stack.pop().unwrap();
    // let this_class_name = get_class_name(&frame.class);
    // let this_class = get_or_load_class(&this_class_name);

    // let object_class = match sfv {
    //     StackFrameValue::Reference(id) =>{
    //        let reference =  get_reference(&id).unwrap();
    //        match reference {
    //            Reference::Object(object) =>{
    //                 object.class
    //            }
    //            Reference::Array(array) =>{
    //                 //todo 处理数组的情况
    //                 0
    //            }
    //        }
    //     }
    //     _=> panic!()
    // };

    // let target_class_constant: &crate::class::ConstantPoolInfo = this_class
    //     .constant_pool
    //     .get(&u8s_to_u16(&frame.code[(frame.pc + 1)..(frame.pc + 3)])).unwrap();
    // match target_class_constant {
    //     ConstantPoolInfo::Class(utf8_index) =>{
    //         let target_class_utf8_constant = this_class.constant_pool.get(utf8_index).unwrap();
    //         match target_class_utf8_constant {
    //             ConstantPoolInfo::Utf8(utf8) =>{
    //                let class =  get_or_load_class(utf8);
    //                if object_class == class.id {
    //                     frame.op_stack.push(sfv)
    //                }else {
    //                    panic!();
    //                }
    //             }
    //             _=> panic!()
    //         }
    //     }
    //     _=> panic!()
    // }
    
    frame.pc += 3;
    Ok(())
}
