    use std::cell::UnsafeCell;
    use std::sync::Mutex;
    use lazy_static::lazy_static;
    use std::collections::HashMap;
    use crate::array::array::Array;
    use crate::object::Object;
    use crate::class::{Class, MethodInfo};
    use crate::reference::reference::Reference;
    use crate::stack_frame::StackFrame;
    use crate::class_loader::class_loader::load_class;
    use crate::value::value::StackFrameValue;
    use crate::param::param::DataType;
    lazy_static! {
        // 创建一个包含UnsafeCell的Mutex，用于包装全局变量
        //类常量池
        pub static ref CLASS_DATA: Mutex<UnsafeCell<HashMap<String, Class>>> = Mutex::new(UnsafeCell::new(HashMap::new()));
        
        pub static ref CLASS_ID_DATA: Mutex<UnsafeCell<HashMap<usize,String>>> = Mutex::new(UnsafeCell::new(HashMap::new()));
        
        //方法数据，便于调用方法时快速查找
        pub static ref METHOD_DATA: Mutex<UnsafeCell<HashMap<String,MethodInfo>>> = Mutex::new(UnsafeCell::new(HashMap::new()));
        
        //字符串常量池
        pub static ref STR_POOL: Mutex<UnsafeCell<HashMap<Vec<u8>, u32>>> = Mutex::new(UnsafeCell::new(HashMap::new()));
    
        //对象存储
        pub static ref OBJECT_DATA: Mutex<UnsafeCell<HashMap<u32, Reference>>> = Mutex::new(UnsafeCell::new(HashMap::new()));
    
        //对象ID游标
        pub static ref OBJECT_ID: Mutex<UnsafeCell<u32>> = Mutex::new(UnsafeCell::new(0));
    
        //虚拟机栈
        pub static ref VM_STACKS: Mutex<UnsafeCell<HashMap<u32, Vec<StackFrame>>>> = Mutex::new(UnsafeCell::new(HashMap::new()));
        
    }

    pub fn class_exists(class_name:String) -> bool {
        // 获取全局变量的Mutex锁
        let data: std::sync::MutexGuard<'_, UnsafeCell<HashMap<String, Class>>> =
            CLASS_DATA.lock().unwrap();
        unsafe {
            // 从 UnsafeCell 中获取 HashMap 的可变引用
            let map = &mut *data.get();
            // 释放Mutex锁
            drop(data);
            return map.contains_key(&class_name);
        }
    }

    pub fn create_object<'a>(class: usize) -> &'a mut Reference {
        let data = OBJECT_DATA.lock().unwrap();
        let obj_id_data = OBJECT_ID.lock().unwrap();
        let obj;
        let obj_id: &mut u32;
        unsafe {
            let map = &mut *data.get();
            obj_id = &mut *obj_id_data.get();
            obj = Object::new(*obj_id + 1, class);
            map.insert(obj.id, Reference::Object(obj.clone()));
            drop(data);
            drop(obj_id_data);
            *obj_id += 1;
            return  map.get_mut(&obj.id).unwrap();
        }
    }

    pub fn create_array<'a>(len:u32,array_type:DataType) -> &'a mut Reference {
        let data = OBJECT_DATA.lock().unwrap();
        let obj_id_data = OBJECT_ID.lock().unwrap();
        let array;
        let obj_id: &mut u32;
        let mut next_id:u32 = 0;
        unsafe {
            let map = &mut *data.get();
            obj_id = &mut *obj_id_data.get();
            next_id = *obj_id + 1;
            array = Array::new(next_id, len,array_type);
            map.insert(array.id, Reference::Array(array));
            drop(data);
            drop(obj_id_data);
            *obj_id += 1;
            return map.get_mut(&next_id).unwrap();
        }
    }

    /**
     * @id java对象的id
     * 返回引用类型数据
     */
    pub fn get_reference<'a>(id: &u32) -> &'a mut Reference {
        let data = OBJECT_DATA.lock().unwrap();
        unsafe {
            let map = &mut *data.get();
            return map.get_mut(id).unwrap();
        }
    }
    /**
     * @class_name java类名称
     */
    pub fn get_or_load_class<'a>(class_name: &String) -> &'a mut Class {
        let data: std::sync::MutexGuard<'_, UnsafeCell<HashMap<String, Class>>> =
            CLASS_DATA.lock().unwrap();
        unsafe {
            let map = &mut *data.get();
            if  !map.contains_key(class_name)  {
                let mut class = load_class(class_name);
                class.id = map.len() + 1 as usize;
                map.insert(class_name.clone(), class);
                add_id_class(map.len(), class_name.clone());
            }
            drop(data);
            return map.get_mut(class_name).unwrap();
        }
    }

    pub fn add_id_class(class_id: usize, class_name: String) {
        let data: std::sync::MutexGuard<'_, UnsafeCell<HashMap<usize, String>>> =
            CLASS_ID_DATA.lock().unwrap();
        unsafe {
            let map = &mut *data.get();
            map.insert(class_id, class_name);
        }
        drop(data);
    }

    pub fn get_class_name(class_id: &usize) -> String {
        let data: std::sync::MutexGuard<'_, UnsafeCell<HashMap<usize, String>>> =
            CLASS_ID_DATA.lock().unwrap();
        unsafe {
            let map = &mut *data.get();
            drop(data);
            return map.get(class_id).unwrap().clone();
        }
    }


    pub fn add_method(method_info :MethodInfo) {
        let data: std::sync::MutexGuard<'_, UnsafeCell<HashMap<String, MethodInfo>>> =
        METHOD_DATA.lock().unwrap();
        unsafe {
            let key = format!("{}{}{}{}{}", method_info.class_name,".", method_info.method_name,".", method_info.descriptor);
            let map = &mut *data.get();
            map.insert(key, method_info);
        }
        drop(data);
    }

    pub fn get_method_from_pool<'a>(class_name: String,method_name:String,descriptor :String) ->&'a MethodInfo {
        let data: std::sync::MutexGuard<'_, UnsafeCell<HashMap<String, MethodInfo>>> =
        METHOD_DATA.lock().unwrap();
        unsafe {
            let key = format!("{}{}{}{}{}", class_name,".", method_name,".", descriptor);
            let map = &mut *data.get();
            drop(data);
            return  map.get(&key).unwrap();
        }
    }


    pub fn pop_stack_frame(vm_stack_id: u32) {
        let data: std::sync::MutexGuard<'_, UnsafeCell<HashMap<u32, Vec<StackFrame>>>> =
            VM_STACKS.lock().unwrap();
        unsafe {
            let map = &mut *data.get();
            map.get_mut(&vm_stack_id).unwrap().pop();
        }
        drop(data);
    }

    pub fn push_frame_data(vm_stack_id: u32, value:StackFrameValue) {
        let data: std::sync::MutexGuard<'_, UnsafeCell<HashMap<u32, Vec<StackFrame>>>> =
            VM_STACKS.lock().unwrap();
        unsafe {
            let map = &mut *data.get();
            //println!("before push_frame_data：{:?}",&map);
            let l = map.get_mut(&vm_stack_id).unwrap();
            //let len = l.len();
            l.get_mut(0).unwrap().op_stack.push(value);
            //println!("after push_frame_data：{:?}",&map);
        }
        drop(data);
    }

