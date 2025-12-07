
use std::env;
use log::info;
use tomato::{interpreter::instructions::op_code::{self, op_code::*}, runtime::global::VirtualMachine};
fn main() {
    unsafe { env::set_var("RUST_LOG", "DEBUG") };
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
     let mut vm: VirtualMachine =  VirtualMachine::new();
     let thread_id = vm.create_thread(); 
     let mut vm_stack = vm.stack_manager.get_stack_frames_mut().get(&thread_id).unwrap();
     vm.metaspace().load_class( thread_id,&main_class_path,&mut vm.heap,&mut vm.metaspace,&mut vm.stack_manager);
     let main_method_info = vm.metaspace().get_method(&main_class_path, "main", "([Ljava/lang/String;)V");
     if vm.stack_manager.create_stack_frame(thread_id, main_method_info.unwrap()){
        op_code::op_code::run(thread_id,&mut vm.heap,&mut vm.metaspace,&mut vm_stack);
     }
}