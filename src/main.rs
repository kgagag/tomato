pub mod class;
pub mod class_loader;
pub mod u8c;
pub mod stack_frame;
pub mod op_code;
pub mod object;
pub mod runtime_data_area;
pub mod value;
pub mod param;
use std::cell::UnsafeCell;
use crate::stack_frame::stack_frame::StackFrame;
use crate::stack_frame::stack_frame::create_stack_frame;
use crate::runtime_data_area::runtime_data_area::get_or_load_class;
use crate::op_code::op_code::push_stack_frame;
use std::collections::HashMap;
use crate::op_code::op_code::do_opcode;
use crate::runtime_data_area::runtime_data_area::VM_STACKS;


pub fn execute(){
    let data: std::sync::MutexGuard<'_, UnsafeCell<HashMap<u32, Vec<StackFrame>>>> = VM_STACKS.lock().unwrap();
    unsafe {
        // 从 UnsafeCell 中获取 HashMap 的可变引用
        let map = &mut *data.get();
        drop(data);
        for (_vm_stack_id, stack_frames) in map {
            //这里可以启动一个线程
            for _i in 0 .. stack_frames.len() as usize{
                do_opcode(stack_frames);
            }
        }
        
    }
}



fn main() {
    run(String::from("Test2"));
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
