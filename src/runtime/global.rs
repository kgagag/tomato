use std::collections::HashMap;
use std::sync::{Mutex, RwLock, RwLockReadGuard, RwLockWriteGuard};
use lazy_static::lazy_static;

use super::metaspace::Metaspace;
use super::heap::Heap;
use crate::common::stack_frame::{self, StackFrame};
use crate::classfile::class::{Class, MethodInfo};
use crate::common::value::StackFrameValue;
use crate::common::reference::Reference;
use crate::runtime::{heap, metaspace};

/// 虚拟机栈管理器
/// 
/// 负责管理所有线程的调用栈
#[derive(Debug)]
pub struct VMStackManager {
    /// 线程栈存储：线程ID -> 栈帧列表
    stacks: RwLock<HashMap<u64, Vec<StackFrame>>>,
    
    /// 线程ID计数器
    thread_counter: Mutex<u64>,
}

impl VMStackManager {
    /// 创建新的虚拟机栈管理器
    pub fn new() -> Self {
        VMStackManager {
            stacks: RwLock::new(HashMap::new()),
            thread_counter: Mutex::new(0),
        }
    }
    
    /// 创建新线程并分配线程ID
    /// 
    /// # 返回
    /// * `u32` - 新的线程ID
    pub fn create_thread(&self) -> u64 {
        let mut counter = self.thread_counter.lock().unwrap();
        *counter += 1;
        let thread_id = *counter;
        // 初始化线程栈
        let mut stacks = self.stacks.write().unwrap();
        stacks.insert(thread_id, Vec::new());
        thread_id
    }
    

    //通过线程id查找所以栈帧,返回可变引用
    pub fn get_stack_frames_mut(&self) -> RwLockWriteGuard<'_, HashMap<u64, Vec<StackFrame>>> {
        self.stacks.write().unwrap()
    }

    
    //通过线程id查找所以栈帧,返回可变引用
    pub fn get_stack_frames(&self) -> RwLockReadGuard<'_, HashMap<u64, Vec<StackFrame>>> {
        self.stacks.read().unwrap()
    }

    

    /// 销毁线程及其栈
    /// 
    /// # 参数
    /// * `thread_id` - 要销毁的线程ID
    pub fn destroy_thread(&self, thread_id: u64) {
        let mut stacks = self.stacks.write().unwrap();
        stacks.remove(&thread_id);
    }
    
    /// 推入新的栈帧
    /// 
    /// # 参数
    /// * `thread_id` - 线程ID
    /// * `frame` - 要推入的栈帧
    /// 
    /// # 返回
    /// * `bool` - 操作是否成功
    pub fn push_frame(&self, thread_id: u64, frame: StackFrame) -> bool {
        let mut stacks = self.stacks.write().unwrap();
        if let Some(frames) = stacks.get_mut(&thread_id) {
            frames.push(frame);
            true
        } else {
            false
        }
    }
    
    /// 弹出当前栈帧
    /// 
    /// # 参数
    /// * `thread_id` - 线程ID
    /// 
    /// # 返回
    /// * `Option<StackFrame>` - 弹出的栈帧（如果存在）
    pub fn pop_frame(&self, thread_id: u64) -> Option<StackFrame> {
        let mut stacks = self.stacks.write().unwrap();
        if let Some(frames) = stacks.get_mut(&thread_id) {
            frames.pop()
        } else {
            None
        }
    }
        
    /// 从当前栈帧的操作数栈弹出值
    /// 
    /// # 参数
    /// * `thread_id` - 线程ID
    /// 
    /// # 返回
    /// * `Option<StackFrameValue>` - 弹出的值
    pub fn pop_operand(&self, thread_id: u64) -> Option<StackFrameValue> {
        let mut stacks = self.stacks.write().unwrap();
        if let Some(frames) = stacks.get_mut(&thread_id) {
            if let Some(frame) = frames.last_mut() {
                return frame.op_stack.pop();
            }
        }
        None
    }
    
    /// 获取当前栈帧深度（当前栈中的帧数）
    /// 
    /// # 参数
    /// * `thread_id` - 线程ID
    /// 
    /// # 返回
    /// * `Option<usize>` - 栈深度（如果线程存在）
    pub fn stack_depth(&self, thread_id: u64) -> Option<usize> {
        let stacks = self.stacks.read().unwrap();
        stacks.get(&thread_id).map(|frames| frames.len())
    }
    
    /// 检查线程是否存在
    /// 
    /// # 参数
    /// * `thread_id` - 线程ID
    /// 
    /// # 返回
    /// * `bool` - 线程是否存在
    pub fn thread_exists(&self, thread_id: u64) -> bool {
        let stacks = self.stacks.read().unwrap();
        stacks.contains_key(&thread_id)
    }
    
    /// 获取所有活动线程ID
    /// 
    /// # 返回
    /// * `Vec<u32>` - 活动线程ID列表
    pub fn active_threads(&self) -> Vec<u64> {
        let stacks = self.stacks.read().unwrap();
        stacks.keys().cloned().collect()
    }
    
    /// 获取线程栈的统计信息
    /// 
    /// # 返回
    /// * `VMStackStats` - 虚拟机栈统计信息
    pub fn get_stats(&self) -> VMStackStats {
        let stacks = self.stacks.read().unwrap();
        let active_threads = stacks.len();
        
        // 计算总栈帧数
        let total_frames: usize = stacks.values().map(|frames| frames.len()).sum();
        
        // 计算最大栈深度
        let max_stack_depth = stacks.values().map(|frames| frames.len()).max().unwrap_or(0);
        
        VMStackStats {
            active_threads,
            total_frames,
            max_stack_depth,
        }
    }


    /// 执行方法调用（推入新的栈帧）
    /// 
    /// # 参数
    /// * `thread_id` - 线程ID
    /// 、* `method_info` - 要调用的方法信息
    /// # 返回
    /// * `bool` - 调用是否成功
    pub fn create_stack_frame(
        &self,
        thread_id: u64,
        method_info : MethodInfo
    ) -> bool {
        // 创建新的栈帧
        let mut frame =  stack_frame::create_stack_frame(&method_info).unwrap();
        //分配线程id
        frame.vm_stack_id = thread_id;
        // 推入栈帧
        self.push_frame(thread_id, frame)
    }
    

    /// 执行方法调用（推入新的栈帧）
    /// 
    /// # 参数
    /// * `vm_stack` - 虚拟机栈
    /// * `thread_id` - 方法所属类名
    /// * `method_info` - 方法描述符
    /// 
    /// # 返回
    pub fn push_new_stack_frame(
        vm_stack:& mut Vec<StackFrame>,
        thread_id: u64,
        method_info : MethodInfo
    ){
        // 创建新的栈帧
        let mut frame =  stack_frame::create_stack_frame(&method_info).unwrap();
        //分配线程id
        frame.vm_stack_id = thread_id;
        // 推入栈帧
        vm_stack.push(frame)
    }
    


    /// 执行方法返回（弹出当前栈帧）
    /// 
    /// # 参数
    /// * `thread_id` - 线程ID
    /// 
    /// # 返回
    /// * `Option<StackFrame>` - 被弹出的栈帧
    pub fn return_from_method(&self, thread_id: u64) -> Option<StackFrame> {
        self.pop_frame(thread_id)
    }

}

/// 虚拟机栈统计信息
#[derive(Debug, Clone)]
pub struct VMStackStats {
    /// 活动线程数量
    pub active_threads: usize,
    
    /// 总栈帧数量
    pub total_frames: usize,
    
    /// 最大栈深度
    pub max_stack_depth: usize,
}

/// 全局虚拟机运行时管理器
/// 
/// 整合元空间、堆和虚拟机栈，提供统一的访问接口
pub struct VirtualMachine {
    /// 元空间管理器
    pub metaspace: Metaspace,
    
    /// 堆管理器
    pub heap: Heap,
    
    /// 虚拟机栈管理器
    pub stack_manager: VMStackManager,
}

impl VirtualMachine {
    /// 创建新的虚拟机实例
    pub fn new() -> Self {
        VirtualMachine {
            metaspace: Metaspace::new(),
            heap: Heap::new(),
            stack_manager: VMStackManager::new(),
        }
    }
    
    /// 获取元空间管理器引用
    pub fn metaspace(&self) -> &Metaspace {
        &self.metaspace
    }
    
    /// 获取堆管理器引用
    pub fn heap(&self) -> &Heap {
        &self.heap
    }
    
    /// 获取虚拟机栈管理器引用
    pub fn stack_manager(&self) -> &VMStackManager {
        &self.stack_manager
    }
        
 
    
    /// 创建新线程
    /// 
    /// # 返回
    /// * `u32` - 新线程的ID
    pub fn create_thread(&self) -> u64 {
        self.stack_manager.create_thread()
    }
    
}