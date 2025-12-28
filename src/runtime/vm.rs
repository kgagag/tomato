use std::{collections::HashMap};

use crate::{common::stack_frame::StackFrame, runtime::{heap::Heap, metaspace::Metaspace}};
use lazy_static::lazy_static;

pub struct Vm{
    pub heap: Heap, 
    pub metaspace: Metaspace,
    pub vm_stack: HashMap<u8,Vec<StackFrame>>,
}

impl Vm {

    //创建虚拟机
    pub fn create() -> Vm {
        Vm {
            heap: Heap::create(),
            metaspace: Metaspace::create(),
            vm_stack: HashMap::new()
        }
    }

    //将栈帧压入虚拟机栈,遍历所有key，找到第一个不存在的key
    pub fn push_stack_frame(&mut self, frame: StackFrame) -> u8 {
        for i in 0..=u8::MAX {
            if !self.vm_stack.contains_key(&i) {
                self.vm_stack.insert(i, vec![frame]);
                return i;
            }
        }
        panic!("vm stack is full");
    }
   
}