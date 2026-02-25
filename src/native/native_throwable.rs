use crate::{
    classloader::class_loader, common::{error::Throwable, stack_frame::StackFrame, value::StackFrameValue}, runtime::{heap::Heap, metaspace::Metaspace}
};

pub fn fill_in_stack_trace( vm_stack: &mut Vec<StackFrame>,
    heap: &mut Heap,
    metaspace: &mut Metaspace) -> Result<(), Throwable> {
    // 获取当前正在处理的 throwable 对象 (this)
    //let this_obj = frame.local[0];  // this 引用
    
    // 在 Java 的 Throwable 类中，fillInStackTrace 的目的是填充当前线程的堆栈跟踪信息
    // 这个方法需要获取当前线程的执行堆栈并将其保存到 Throwable 对象中
    // 在我们的实现中，我们只需将 this 对象放回操作数栈作为返回值
    // 因为 Java 代码中的 fillInStackTrace() 方法会处理具体的字段设置逻辑
    
    // 从当前的调用堆栈中获取堆栈跟踪信息
    // 在真实的 JVM 中，这会通过 native 方法来完成
    // 这里我们只需要确保 this 对象被正确返回
    
    // 将 this 对象压入操作数栈作为返回值（因为 fillInStackTrace 返回 Throwable）
    //frame.op_stack.push(this_obj);
    
    // 在真正的实现中，这里会获取当前执行堆栈的跟踪信息并存储到 Throwable 对象中
    // 但现在我们只是简单地返回 this 对象，因为 Java 端的 fillInStackTrace() 方法
    // 会负责设置 stackTrace 和 backtrace 字段
    

    let frame_index = vm_stack.len() - 1;
    let stfv = vm_stack[frame_index].op_stack.pop().unwrap();
    match stfv {
        StackFrameValue::Reference(id) => {
            let class_name = "java/lang/StackTraceElement".to_string();
            let class = class_loader::find_class(&class_name, vm_stack, heap, metaspace)?;
            let object_id = heap.create_reference_array(class.super_class_id as u32, 0 , 0, 12);
            // heap.create_object(class)
            // heap.put_field_reference(reference_id, offset, value);
        }
        _ => panic!(),
    }
    Ok(())
}