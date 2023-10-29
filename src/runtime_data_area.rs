pub mod runtime_data_area{
    use std::cell::UnsafeCell;
    use std::sync::Mutex;
    use lazy_static::lazy_static;
    use std::collections::HashMap;
    use crate::class::class::Class;
    use crate::object::object::Object;
    use crate::stack_frame::stack_frame::StackFrame;

    lazy_static! {
        // 创建一个包含UnsafeCell的Mutex，用于包装全局变量
        //类常量池
        pub static ref CLASS_DATA: Mutex<UnsafeCell<HashMap<Vec<u8>, Class>>> = Mutex::new(UnsafeCell::new(HashMap::new()));
    
        pub static ref CLASS_ID_DATA: Mutex<UnsafeCell<HashMap<usize, Vec<u8>>>> = Mutex::new(UnsafeCell::new(HashMap::new()));
    
        //字符串常量池
        pub static ref STR_POOL: Mutex<UnsafeCell<HashMap<Vec<u8>, u32>>> = Mutex::new(UnsafeCell::new(HashMap::new()));
    
        //对象存储
        pub static ref OBJECT_DATA: Mutex<UnsafeCell<HashMap<u32, Object>>> = Mutex::new(UnsafeCell::new(HashMap::new()));
    
        //对象ID游标
        pub static ref OBJECT_ID: Mutex<UnsafeCell<u32>> = Mutex::new(UnsafeCell::new(0));
    
        //虚拟机栈
        pub static ref VM_STACKS: Mutex<UnsafeCell<HashMap<u32, Vec<StackFrame>>>> = Mutex::new(UnsafeCell::new(HashMap::new()));
        
    }
}