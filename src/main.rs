pub mod class;
pub mod class_loader;
pub mod u8c;
pub mod stack_frame;
pub mod op_code;
pub mod object;
pub mod runtime_data_area;
pub mod value;
pub mod param;
pub mod reference;
pub mod array;
pub mod opcode_store;
pub mod opcode_array;
pub mod opcode_const;
pub mod opcode_push;
pub mod opcode_nop;
pub mod opcode_load;
pub mod opcode_ldc;
pub mod opcode_math;
pub mod opcode_return;
pub mod opcode_dup;
pub mod opcode_pop;
pub mod opcode_swap;
pub mod opcode_convert;
pub mod opcode_compare;
pub mod opcode_goto;
pub mod opcode_static;
pub mod opcode_field;
pub mod opcode_new;
pub mod opcode_invoke;
pub mod native;
pub mod native_io;
use std::cell::UnsafeCell;
use crate::class::ConstantPoolInfo;
use crate::stack_frame::*;
use crate::runtime_data_area::get_or_load_class;
use std::collections::HashMap;
use crate::op_code::op_code::do_opcode;
use crate::runtime_data_area::VM_STACKS;
extern crate log;
extern crate env_logger;
use crate::op_code::op_code::*;
use std::env;
use log::{error, info, warn};
use crate::opcode_invoke::*;





fn main() {
    env::set_var("RUST_LOG", "DEBUG");
    env_logger::Builder::from_default_env()
        .format_timestamp(Some(env_logger::TimestampPrecision::Millis))
        .format_module_path(true)
        .init();
    run(String::from("Test"));
}

/***
 * 虚拟机启动方法
 */
pub fn run(main_class_path: String) {
    let class = get_or_load_class(&main_class_path);   
    //创建VM
    //找到main方法
    for i in 0..* &class.method_info.len() {
        let method_info = &class.method_info[i];
        //let methond_index = (method_info.name_index as usize) - 1;
        let u8_vec = class.constant_pool.get(&method_info.name_index).unwrap();
        match u8_vec {
            ConstantPoolInfo::Utf8(name) =>{
                //println!("method:{}", &name);
                info!("{}", name);
                //创建虚拟机栈，并创建第一个栈帧
                if name == "main" {
                    let stack_frame = create_stack_frame(method_info).unwrap();
                    let stack_frame_clone = stack_frame.clone();
                    let vm_stack_id = push_stack_frame(stack_frame);
                    execute(vm_stack_id);
                }
            }
            _=> panic!("wrong class data")
        }
    }
}