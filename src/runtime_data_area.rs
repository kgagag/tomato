pub mod runtime_data_area{
    use std::cell::UnsafeCell;
    use std::sync::Mutex;
    use lazy_static::lazy_static;
    use std::collections::HashMap;
    use crate::class::class::Class;
    use crate::object::object::Object;
    use crate::stack_frame::stack_frame::StackFrame;
    use crate::class_loader::class_loader::load_class;
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

    pub fn class_exists(class_name: Vec<u8>) -> bool {
        // 获取全局变量的Mutex锁
        let data: std::sync::MutexGuard<'_, UnsafeCell<HashMap<Vec<u8>, Class>>> =
            CLASS_DATA.lock().unwrap();
        unsafe {
            // 从 UnsafeCell 中获取 HashMap 的可变引用
            let map = &mut *data.get();
            // 释放Mutex锁
            drop(data);
            return map.contains_key(&class_name);
        }
    }

    pub fn create_object<'a>(class: usize) -> &'a mut Object {
        // 获取全局变量的Mutex锁
        let data = OBJECT_DATA.lock().unwrap();
        let obj_id_data = OBJECT_ID.lock().unwrap();

        // 通过UnsafeCell获取可变引用，并修改全局变量的值
        let obj;
        let obj_id: &mut u32;
        unsafe {
            let map = &mut *data.get();
            obj_id = &mut *obj_id_data.get();
            obj = Object::new(*obj_id + 1, class);
            map.insert(obj.id, obj.clone());
            drop(data);
            drop(obj_id_data);
            *obj_id += 1;
            return map.get_mut(&obj.id).unwrap();
        }
    }

    pub fn get_object<'a>(id: &u32) -> &'a mut Object {
        // 获取全局变量的Mutex锁
        let data = OBJECT_DATA.lock().unwrap();
        // 通过UnsafeCell获取可变引用，并修改全局变量的值
        unsafe {
            let map = &mut *data.get();
            return map.get_mut(id).unwrap();
        }
    }
    pub fn get_or_load_class<'a>(class_name: &Vec<u8>) -> &'a mut Class {
        // 获取全局变量的Mutex锁
        let data: std::sync::MutexGuard<'_, UnsafeCell<HashMap<Vec<u8>, Class>>> =
            CLASS_DATA.lock().unwrap();
        // 通过UnsafeCell获取可变引用，并修改全局变量的值
        unsafe {
            // 从 UnsafeCell 中获取 HashMap 的可变引用
            let map = &mut *data.get();
            // 释放Mutex锁
            if  !map.contains_key(class_name)  {
                let mut class = load_class(&(String::from_utf8(class_name.clone()).unwrap()));
                class.id = map.len() + 1 as usize;
                map.insert(class_name.clone(), class);
                add_id_class(map.len(), class_name.clone());
            }
            drop(data);
            return map.get_mut(class_name).unwrap();
        }
    }

    pub fn add_id_class(class_id: usize, class_name: Vec<u8>) {
        // 获取全局变量的Mutex锁
        let data: std::sync::MutexGuard<'_, UnsafeCell<HashMap<usize, Vec<u8>>>> =
            CLASS_ID_DATA.lock().unwrap();
        // 通过UnsafeCell获取可变引用，并修改全局变量的值
        unsafe {
            // 从 UnsafeCell 中获取 HashMap 的可变引用
            let map = &mut *data.get();
            // 添加或修改键值对
            map.insert(class_id, class_name);
        }
        // 释放Mutex锁
        drop(data);
    }

    pub fn get_class_name(class_id: &usize) -> Vec<u8> {
        // 获取全局变量的Mutex锁
        let data: std::sync::MutexGuard<'_, UnsafeCell<HashMap<usize, Vec<u8>>>> =
            CLASS_ID_DATA.lock().unwrap();
        // 通过UnsafeCell获取可变引用，并修改全局变量的值
        unsafe {
            // 从 UnsafeCell 中获取 HashMap 的可变引用
            let map = &mut *data.get();
            // 释放Mutex锁
            drop(data);
            return map.get(class_id).unwrap().clone();
        }
    }


}