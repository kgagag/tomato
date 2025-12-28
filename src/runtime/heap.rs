use std::{collections::HashMap, f32::consts::E};

use dashmap::DashMap;
use log::info;

use crate::{classfile::class::Class, common::param::DataType, memory, utils::u8c};

pub struct Heap {
    memory: Vec<u8>,
    memory_block: Vec<(u32, u32)>,
    address_map: Vec<u32>,
    address_map_index: usize,
    address_malloc_method: u8,
    str_pool: HashMap<String, u32>,
    class_pool: HashMap<String, u32>,
}

impl Heap {
    //创建堆
    pub fn create() -> Heap {
        Heap {
            memory: vec![0u8; 1024 * 1024 ],
            address_map: vec![0u32; 1024 * 1024 ],
            memory_block: vec![(0, 1024 * 1024)],
            address_map_index: 0,
            address_malloc_method: 0,
            str_pool: HashMap::new(),
            class_pool: HashMap::new(),
        }
    }

    //分配内存,如果内存块足够，则分配成功,更新内存块信息
    pub fn malloc(&mut self, size: u32, is_array: bool, class_id: u32) -> usize {
        let index = self.address_map_index;
        for i in 0..self.memory_block.len() {
            let (address, block_size) = self.memory_block.get(i).unwrap().clone();
            if block_size >= size {
                let new_address: u32 = address + size;
                let new_block_size = block_size - size;
                self.memory_block.remove(i);
                if new_block_size > 0 {
                    self.memory_block[i] = (new_address, new_block_size);
                } else {
                    self.memory_block.remove(i);
                }
                self.address_map[self.address_map_index] = address;
                if self.address_malloc_method == 0 {
                    if self.address_map_index < (self.address_map.len() - 1) {
                        self.address_map_index += 1;
                    } else {
                        self.address_malloc_method = 1;
                        //在address_map 中寻找指向0的位置
                        for j in 0..self.address_map.len() {
                            if self.address_map[j] == 0 {
                                self.address_map_index = j;
                                break;
                            }
                        }
                    }
                } else {
                    //在address_map 中寻找指向0的位置
                    for j in 0..self.address_map.len() {
                        if self.address_map[j] == 0 {
                            self.address_map_index = j;
                            break;
                        }
                    }
                }
            }
            panic!("heap out of memory");
        }

        if is_array {
            //第1个字节第1位设置为1
            self.memory[index] = 0b10000000;
        }

        //设置class_id(从第3个u8开始)
        let cid = u8c::split_u32_to_u8(class_id);
        self.memory[index + 0x2] = cid[0x0];
        self.memory[index + 0x3] = cid[0x1];
        self.memory[index + 0x4] = cid[0x2];
        self.memory[index + 0x5] = cid[0x3];

        index
    }

    pub fn create_object(&mut self, class: &mut Class) -> usize {
        let mut size: u32 = 0;
        for (_key, value) in &class.field_info {
            match &value.data_type {
                DataType::Byte => {
                    size += 1;
                }
                DataType::Char => {
                    size += 2;
                }
                DataType::Float => {
                    size += 4;
                }
                DataType::Double => {
                    size += 8;
                }
                DataType::Int => {
                    size += 4;
                }
                DataType::Long => {
                    size += 8;
                }
                DataType::Reference(id) => {
                    // 修正：使用正确的格式化语法
                    info!("Reference id: {}", id);
                    size += 8;
                }
                DataType::Short => {
                    size += 2;
                }
                DataType::Boolean => {
                    size += 1;
                }
                _ => panic!(),
            }
        }
        //分配内存
        self.malloc(size, false, class.id as u32)
    }
}

// 设计对象

//非数组
// 对象头（2个字节，第1位 0） class_id（4个字节）+ 对象数据 + 对齐

//数组
// 对象头（2个字节，第1位 1） ...
