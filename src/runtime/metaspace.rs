use std::collections::HashMap;
use std::sync::{RwLock, Mutex};

use crate::classfile::class::{Class, MethodInfo};
use crate::classloader;
use crate::classloader::class_loader::load_class;
use crate::common::stack_frame::StackFrame;
use crate::runtime::global::{VMStackManager, VirtualMachine};
use crate::runtime::heap::Heap;
use crate::runtime::metaspace;

/// 元空间管理器
/// 
/// 负责管理虚拟机的元数据，包括：
/// - 类定义存储
/// - 方法信息存储
/// - 常量池管理等
/// 
/// 元空间中的数据在类卸载前一直存在
pub struct Metaspace {
    /// 类定义存储：类名 -> 类定义
    class_data: RwLock<HashMap<String, Class>>,
    
    /// 方法定义存储：方法签名 -> 方法信息
    method_data: RwLock<HashMap<String, MethodInfo>>,
    
    /// 字符串常量池：字符串 -> 对象ID
    str_constant_pool: RwLock<HashMap<String, u64>>,
    
    /// 类常量池：类名 -> 类对象ID
    class_constant_pool: RwLock<HashMap<String, u64>>,
}

impl Metaspace {
    /// 创建新的元空间实例
    pub fn new() -> Self {
        Metaspace {
            class_data: RwLock::new(HashMap::new()),
            method_data: RwLock::new(HashMap::new()),
            str_constant_pool: RwLock::new(HashMap::new()),
            class_constant_pool: RwLock::new(HashMap::new()),
        }
    }
    
    // ==================== 类管理 ====================
    
    /// 检查指定类是否已加载
    /// 
    /// # 参数
    /// * `class_name` - 类名
    /// 
    /// # 返回
    /// * `bool` - 类是否已加载
    pub fn class_exists(&self, class_name: &str) -> bool {
        let class_map = self.class_data.read().unwrap();
        class_map.contains_key(class_name)
    }
    
    /// 获取已加载的类定义
    /// 
    /// # 参数
    /// * `class_name` - 类名
    /// 
    /// # 返回
    /// * `Option<Class>` - 类定义（如果已加载）
    pub fn get_class(&self, class_name: &str) -> Option<Class> {
        let class_map = self.class_data.read().unwrap();
        class_map.get(class_name).cloned()
    }
    
    /// 注册新的类定义
    /// 
    /// # 参数
    /// * `class` - 要注册的类定义
    /// 
    /// # 返回
    /// * `Class` - 注册后的类定义（包含分配的类ID）
    pub fn register_class(&self, mut class: Class) -> Class {
        let mut class_map = self.class_data.write().unwrap();
        let class_name = class.class_name.clone();
        
        // 如果类尚未注册
        if !class_map.contains_key(&class_name) {
            // 分配类ID（从1开始）
            class.id = class_map.len() + 1;
            class_map.insert(class_name.clone(), class.clone());
        }
        
        class_map.get(&class_name).unwrap().clone()
    }
 
    
    /// 获取所有已加载的类名
    /// 
    /// # 返回
    /// * `Vec<String>` - 所有已加载的类名列表
    pub fn get_all_class_names(&self) -> Vec<String> {
        let class_map = self.class_data.read().unwrap();
        class_map.keys().cloned().collect()
    }
    
    // ==================== 方法管理 ====================
    
    /// 注册方法定义
    /// 
    /// # 参数
    /// * `method_info` - 方法信息
    pub fn register_method(&self, method_info: MethodInfo) {
        let key = format!(
            "{}.{}.{}",
            method_info.class_name, method_info.method_name, method_info.descriptor
        );
        let mut method_map = self.method_data.write().unwrap();
        method_map.insert(key, method_info);
    }
    
    /// 根据方法签名查找方法定义
    /// 
    /// # 参数
    /// * `class_name` - 类名
    /// * `method_name` - 方法名
    /// * `descriptor` - 方法描述符
    /// 
    /// # 返回
    /// * `Option<MethodInfo>` - 方法信息（如果存在）
    pub fn get_method(
        &self,
        class_name: &str,
        method_name: &str,
        descriptor: &str,
    ) -> Option<MethodInfo> {
        let key = format!("{}.{}.{}", class_name, method_name, descriptor);
        let method_map = self.method_data.read().unwrap();
        method_map.get(&key).cloned()
    }
    
    /// 获取指定类的所有方法
    /// 
    /// # 参数
    /// * `class_name` - 类名
    /// 
    /// # 返回
    /// * `Vec<MethodInfo>` - 类的方法列表
    pub fn get_class_methods(&self, class_name: &str) -> Vec<MethodInfo> {
        let method_map = self.method_data.read().unwrap();
        method_map
            .iter()
            .filter(|(key, _)| key.starts_with(&format!("{}.", class_name)))
            .map(|(_, method)| method.clone())
            .collect()
    }
    
    // ==================== 常量池管理 ====================
    
    /// 获取字符串常量引用
    /// 
    /// # 参数
    /// * `str` - 字符串值
    /// 
    /// # 返回
    /// * `Option<u64>` - 字符串对象的ID（如果存在）
    pub fn get_string_constant(&self, str: &str) -> Option<u64> {
        let pool = self.str_constant_pool.read().unwrap();
        pool.get(str).copied()
    }
    
    /// 注册字符串常量
    /// 
    /// # 参数
    /// * `string` - 字符串值
    /// * `obj_id` - 字符串对象ID
    pub fn register_string_constant(&self, string: String, obj_id: u64) {
        let mut pool = self.str_constant_pool.write().unwrap();
        pool.insert(string, obj_id);
    }


           /// 创建字符串对象并注册到常量池
    /// 
    /// # 参数
    /// * `string` - 字符串值
    /// 
    /// # 返回
    /// * `u64` - 字符串对象的ID
    pub fn create_string_object(&self, string: String,heap : &mut Heap) -> u64 {
        // 先检查常量池中是否已存在
        if let Some(id) =self.get_string_constant(&string) {
            // 验证对象是否仍然存在
            if heap.object_exists(&id) {
                return id;
            }
        }
        // 常量池中不存在或对象已被回收，创建新对象
        let obj_id = heap.create_object(string.clone());
        // 注册到常量池
        self.register_string_constant(string, obj_id);
        obj_id
    }
    
    /// 获取类对象常量引用
    /// 
    /// # 参数
    /// * `class_name` - 类名
    /// 
    /// # 返回
    /// * `Option<u64>` - 类对象的ID（如果存在）
    pub fn get_class_constant(&self, class_name: &str) -> Option<u64> {
        let pool = self.class_constant_pool.read().unwrap();
        pool.get(class_name).copied()
    }
    
    /// 注册类对象常量
    /// 
    /// # 参数
    /// * `class_name` - 类名
    /// * `obj_id` - 类对象ID
    pub fn register_class_constant(&self, class_name: String, obj_id: u64) {
        let mut pool = self.class_constant_pool.write().unwrap();
        pool.insert(class_name, obj_id);
    }
    
    
    /**
     * @class_name java类名称
     */
    pub fn load_class( &mut self,thread_id : u64,class_name: &str,heap : &mut Heap,vm_tack : &mut Vec<StackFrame>) -> Class {
        {
            let  rmap= self.class_data.read().unwrap();
            let result = rmap.get(class_name);
            if result.is_some() {
                return result.unwrap().clone();
            }
        }
        let class =  classloader::class_loader::load_class(&class_name.to_string(),thread_id,heap, self,vm_tack);
        let mut wmap: std::sync::RwLockWriteGuard<'_, HashMap<String, Class>> = self.class_data.write().unwrap();
        wmap.insert(class_name.to_string(), class.clone());
        return class;
    }

   
    /// 注册方法   
    pub fn add_method(&self,method_info: MethodInfo) {
        let key = format!(
            "{}{}{}{}{}",
            method_info.class_name, ".", method_info.method_name, ".", method_info.descriptor
        );
        let mut map: std::sync::RwLockWriteGuard<'_, HashMap<String, MethodInfo>> = self.method_data.write().unwrap();
        map.insert(key, method_info);
    }

    // ==================== 统计与监控 ====================
    
    /// 获取元空间统计信息
    /// 
    /// # 返回
    /// * `MetaspaceStats` - 元空间统计信息
    pub fn get_stats(&self) -> MetaspaceStats {
        let class_count = self.class_data.read().unwrap().len();
        let method_count = self.method_data.read().unwrap().len();
        let string_pool_size = self.str_constant_pool.read().unwrap().len();
        let class_pool_size = self.class_constant_pool.read().unwrap().len();
        
        MetaspaceStats {
            class_count,
            method_count,
            string_pool_size,
            class_pool_size,
        }
    }
}

/// 元空间统计信息
#[derive(Debug, Clone)]
pub struct MetaspaceStats {
    /// 已加载类数量
    pub class_count: usize,
    
    /// 已注册方法数量
    pub method_count: usize,
    
    /// 字符串常量池大小
    pub string_pool_size: usize,
    
    /// 类常量池大小
    pub class_pool_size: usize,
}