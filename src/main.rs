
use std::env;
use log::info;
use tomato::{classfile::class::ConstantPoolInfo, common::stack_frame::{create_stack_frame, push_stack_frame}, interpreter::instructions::op_code::op_code::execute, runtime::runtime_data_area::get_or_load_class};
fn main() {
    env::set_var("RUST_LOG", "DEBUG");
    env_logger::Builder::from_default_env()
        .format_timestamp(Some(env_logger::TimestampPrecision::Millis))
        .format_module_path(true)
        .init();
    run(String::from("test/Test"));
}

/***
 * 虚拟机启动方法
 */
pub fn run(main_class_path: String) {
    let class = get_or_load_class(&main_class_path);   
    //创建VM
    //找到main方法
    for i in 0..class.method_info.len() {
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
                    let vm_stack_id = push_stack_frame(stack_frame);
                    execute(vm_stack_id);
                }
            }
            _=> panic!("wrong class data")
        }
    }
}