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
            memory: vec![0u8; 1024 * 1024],
            address_map: vec![0u32; 1024 * 1024],
            memory_block: vec![(0, 1024 * 1024)],
            address_map_index: 0,
            address_malloc_method: 0,
            str_pool: HashMap::new(),
            class_pool: HashMap::new(),
        }
    }

    //分配内存,如果内存块足够，则分配成功,更新内存块信息
    pub fn malloc(&mut self, size: u32, is_array: bool, class_id: u32) -> usize {
        // 查找合适的内存块
        let mut delete_index = None;

        for (i, (address, block_size)) in self.memory_block.iter().enumerate() {
            if *block_size >= size {
                delete_index = Some(i);
                break;
            }
        }

        let delete_index = match delete_index {
            Some(idx) => idx,
            None => panic!("heap out of memory"),
        };

        let (address, block_size) = self.memory_block[delete_index].clone();

        // 分割剩余内存块
        let new_address = address + size;
        let new_block_size = block_size - size;

        if new_block_size > 0 {
            self.memory_block.push((new_address, new_block_size));
        }

        // 移除已分配的内存块
        self.memory_block.remove(delete_index);

        // 更新address_map
        let index = self.address_map_index;
        self.address_map[index] = address;

        // 更新address_map_index
        self.update_address_map_index();

        // 设置内存标记
        if is_array {
            self.memory[index] = 0b10000000;
        }

        // 设置class_id
        let cid = u8c::split_u32_to_u8(class_id);
        self.memory[index + 0x2] = cid[0x0];
        self.memory[index + 0x3] = cid[0x1];
        self.memory[index + 0x4] = cid[0x2];
        self.memory[index + 0x5] = cid[0x3];

        index
    }

    // 更新address_map_index
    fn update_address_map_index(&mut self) {
        if self.address_malloc_method == 0 {
            // 顺序分配模式
            if self.address_map_index < self.address_map.len() - 1 {
                self.address_map_index += 1;
            } else {
                // 切换到寻找空闲位置模式
                self.address_malloc_method = 1;
                self.find_next_available_index();
            }
        } else {
            // 寻找空闲位置模式
            self.find_next_available_index();
        }
    }

    // 寻找下一个可用的address_map索引
    fn find_next_available_index(&mut self) {
        for j in 0..self.address_map.len() {
            if self.address_map[j] == 0 {
                self.address_map_index = j;
                return;
            }
        }
        // 如果没有找到空闲位置，可以在这里处理（例如扩展address_map或panic）
        // 暂时保持当前索引不变
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

        if size == 0 {
            size = 0x8;
        } else {
            size = ((size + 7) / 8) * 8;
        }
        self.malloc(size, false, class.id as u32)
    }
}

// 设计对象

//非数组
// 对象头（2个字节，第1位 0） class_id（4个字节）+ 对象数据 + 对齐

//数组
// 对象头（2个字节，第1位 1） ...
