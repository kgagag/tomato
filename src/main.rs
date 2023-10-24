pub mod class;
pub mod class_loader;
pub mod u8c;
pub mod stack_frame;
pub mod op_code;
pub mod object;
pub mod runtime_data_area;
use std::mem;
use std::cell::UnsafeCell;
use std::sync::Mutex;
use lazy_static::lazy_static;
use crate::u8c::u8c::u8s_to_u16;
use crate::u8c::u8c::u8s_to_u32;
use crate::u8c::u8c::u8s_to_u64;
use crate::class_loader::class_loader::load_class;
use crate::class::class::Class;
use crate::class::class::MethodInfo;
use crate::class::class::MethodParameter;
use crate::class::class::CodeAttribute;
use crate::stack_frame::stack_frame::StackFrame;
use crate::op_code::op_code::get_class_name;
use crate::op_code::op_code::get_or_load_class;
use crate::op_code::op_code::push_stack_frame;
use std::collections::HashMap;
use crate::op_code::op_code::do_opcode;
use runtime_data_area::runtime_data_area::VM_STACKS;
pub fn create_stack_frame(
    method_info: &MethodInfo,
    class_id: usize,
) -> Option<StackFrame> {
    let class_name = get_class_name(&class_id);
    let class = get_or_load_class(&class_name);
    for attr in &method_info.attributes {
        let attr_index = (attr.attribute_name_index as usize) - 1;
        let u8_vec = &class.constant_pool[attr_index];
        let slice = &u8_vec[3..u8_vec.len()];
        let name = String::from_utf8(slice.to_vec()).expect("Found invalid UTF-8");
        if "Code" == name {
            //读取Code属性
            //读取 max_stack
            let max_stack = u8s_to_u16(&attr.info[0..2]);
            let max_locals: u16 = u8s_to_u16(&attr.info[2..4]);
            let code_length: u32 = u8s_to_u32(&attr.info[4..8]);
            let mut code: Vec<u8> = Vec::new();
            for c in 0..code_length {
                code.push(attr.info[(c + 8) as usize]);
            }
            let code_attr: CodeAttribute =
                CodeAttribute::new(max_stack, max_locals, code_length, code);
            let mut local: Vec<u32> = Vec::new();
            for _i in 0..max_locals {
                local.push(0);
            }
            return Some( StackFrame::new(
                class_id,
                code_attr.max_stack,
                code_attr.max_locals,
                code_attr.code,
            )
        )
        }
    }
    return None;
}



pub fn execute(){
    let data: std::sync::MutexGuard<'_, UnsafeCell<HashMap<u32, Vec<StackFrame>>>> = VM_STACKS.lock().unwrap();
    unsafe {
        // 从 UnsafeCell 中获取 HashMap 的可变引用
        let map = &mut *data.get();
        drop(data);
        for (vm_stack_id, stack_frames) in map {
            //这里可以启动一个线程
            for i in 0 .. stack_frames.len() as usize{
                do_opcode(stack_frames);
            }
        }
        
    }
}



fn main() {
    run(String::from("Test2"));
}



#[derive(Debug)]
/****栈帧返回值 */
pub struct Ret {
    pub value: Vec<u8>,
    //返回值类型 正常返回 0 异常返回 1
    pub ret_type: u8,
    //返回值长度
    pub length: u8,
}

impl Ret {
    fn new(value: Vec<u8>, ret_type: u8, length: u8) -> Ret {
        Ret {
            value: value,
            ret_type: ret_type,
            length: length,
        }
    }
}


/***
 * 虚拟机启动方法
 */
pub fn run(main_class_path: String) {
    let class = get_or_load_class(&main_class_path.as_bytes().to_vec());   
    //创建VM
    //找到main方法
    for i in 0..* &class.method_info.len() {
        let method_info = &class.method_info[i];
        let methond_index = (method_info.name_index as usize) - 1;
        let u8_vec = &class.constant_pool[methond_index];
        let slice = &u8_vec[3..u8_vec.len()];
        let name = String::from_utf8(slice.to_vec()).expect("Found invalid UTF-8");
        println!("method:{}", &name);
        //创建虚拟机栈，并创建第一个栈帧
        if &name == "main" {
           let stack_frame = create_stack_frame(method_info, class.id).unwrap();
           push_stack_frame(stack_frame);
           execute();
        }
    }
}
