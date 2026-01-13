use log::info;
use std::{env, time::Instant, vec};
use tomato::{
    classloader::class_loader,
    common::{error::Throwable, stack_frame::create_stack_frame},
    interpreter::instructions::op_code::op_code::do_opcode,
    runtime::{heap, metaspace, vm::Vm},
};
fn main() {
    unsafe { env::set_var("RUST_LOG", "DEBUG") };
    env_logger::Builder::from_default_env()
        .format_timestamp(Some(env_logger::TimestampPrecision::Millis))
        .format_module_path(true)
        .init();
    let _ = run(String::from("test/Test106"));
}

/***
 * 虚拟机启动方法
 */
pub fn run(main_class_path: String) -> Result<(), Throwable> {
    let mut vm: Vm = Vm::create();
    let class = class_loader::find_class(
        &main_class_path,
        &mut Vec::new(),
        &mut vm.heap,
        &mut vm.metaspace,
    )?;
    for method_info in &class.method_info {
        if method_info.method_name == "main" && method_info.descriptor == "([Ljava/lang/String;)V" {
            let stack_frame = create_stack_frame(&method_info.clone(), class).unwrap();
            let vm_stack_id = vm.push_stack_frame(stack_frame);
            let vm_stack: &mut Vec<tomato::common::stack_frame::StackFrame> = vm.vm_stack.get_mut(&vm_stack_id).unwrap();
            let heap = &mut vm.heap;
            let metaspace: &mut metaspace::Metaspace = &mut vm.metaspace;
            let start = Instant::now();
            do_opcode(vm_stack, heap, metaspace)?;
            let duration = start.elapsed();
            info!("执行时间: {:?}", duration.as_nanos());
            break;
        }
    }
    Ok(())
}
