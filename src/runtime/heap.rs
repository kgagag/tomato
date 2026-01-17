use std::{collections::HashMap, f32::consts::E};

use dashmap::DashMap;
use log::info;

// 常量定义
const DEFAULT_HEAP_SIZE: usize = 1024 * 1024;
//对象头（2个字节，第1位 1）+ class_id（4个字节） = 6字节
const OBJECT_HEADER_SIZE: u32 = 6;
//数组对象头（2个字节，第1位 0,第 2 位 1）+ 数组长度(4字节） +维度(预留1个字节）+ data_type（1个字节) = 8字节
const ARRAY_HEADER_SIZE_BASIC: u32 = 8;
//对象头（2个字节，第1位 0,第2位 1 如果atype != 12 第3位为0否则为1）  + 数组长度4个字节 +  维度 1个字节   + class_id 4个字节  = 11字节
const ARRAY_HEADER_SIZE_REFERENCE: u32 = 11;

// 标记位定义
const OBJECT_FLAG: u8 = 0b10000000;
//引用类型数组
const REFERENCE_ARRAY_FLAG: u8 = 0b01100000;
//基本类型多维数组
const REFERENCE_ARRAY_MULTI_FLAG: u8 = 0b01000000;

use crate::{
    classfile::class::Class,
    common::{error::Throwable, param::DataType, value},
    memory,
    utils::u8c,
};

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
            memory: vec![0u8; DEFAULT_HEAP_SIZE],
            //设定address_map只会膨胀，不会收缩，每个元素的索引(index)就是当前对象对象的id,address_map[index]指向对象在memory的开始地址
            //大小默认heap_size的四分之一
            address_map: vec![0u32; DEFAULT_HEAP_SIZE / 8],
            //可用内存块列表
            memory_block: vec![(0, 1024 * 1024)],

            address_map_index: 1,
            address_malloc_method: 0,
            str_pool: HashMap::new(),
            class_pool: HashMap::new(),
        }
    }

    /**
     * 分配内存,如果内存块足够，则分配成功,更新内存块信息
     * 返回对象id
     */
    pub fn malloc(&mut self, size: u32) -> usize {
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
        let (address, block_size) = self.memory_block[delete_index];
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

    // 设计对象

    //非数组
    // 对象头（2个字节，第1位 1） class_id（4个字节）+ 对象数据 + 对齐
    pub fn create_object(&mut self, class: &mut Class) -> usize {
        let mut size: u32 = OBJECT_HEADER_SIZE;
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
                DataType::Reference(_id) => {
                    // 修正：使用正确的格式化语法
                    //info!("Reference id: {}", id);
                    size += 4;
                },
               DataType::Array {depth: _depth, element_type: _element_type } => {
                    size += 4;
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

        // 确保size是8的倍数
        if size < 8 {
            size = 8;
        } else {
            size = ((size + 7) / 8) * 8;
        }
        let object_id = self.malloc(size);
        let start = self.address_map[object_id] as usize;

        self.memory[start] = OBJECT_FLAG;
        // 设置class_id
        let cid = u8c::split_u32_to_u8(class.id as u32);
        self.memory[start + 2] = cid[0];
        self.memory[start + 3] = cid[1];
        self.memory[start + 4] = cid[2];
        self.memory[start + 5] = cid[3];

        object_id
    }


   /**
    * 检查一个对象是否是数组
    */
    pub fn is_array(&self, object_id: usize) -> bool {
        let start = self.address_map[object_id] as usize;
        //info!("==============object flag: {:08b}==========", self.memory[start]);
        self.memory[start] & 0b10000000 == 0
    }

    pub fn get_object_class_id(&self, object_id: usize) -> Result<u32,Throwable> {
        let start = self.address_map[object_id] as usize;
        let mut array = [0u8;4];
        array[0] = self.memory[start + 2];
        array[1] = self.memory[start + 3];
        array[2] = self.memory[start + 4];
        array[3] = self.memory[start + 5];
        Ok(u8c::combine_u8_to_u32(
           array
        ))
    }


    /**
     * 创建基本类型数组对象
     * 对象头（2个字节，第1位 0,第 2 位 0）+ 数组长度 4 +维度 1个字 + data_type 1个字节 + 数组数据 + 对齐
     */
    pub fn create_basic_array(&mut self, atype: u8, len: u32, dimension: u8) -> usize {
        //let start_size = ARRAY_HEADER_SIZE_BASIC;
        let size = {
            if atype == 4 {
                // array_type = DataType::Boolean;
                1
            } else if atype == 5 {
                // array_type = DataType::Char;
                2
            } else if atype == 6 {
                // array_type = DataType::Float;
                4
            } else if atype == 7 {
                //  array_type = DataType::Double;
                8
            } else if atype == 8 {
                //  array_type = DataType::Byte;
                1
            } else if atype == 9 {
                // array_type = DataType::Short;
                2
            } else if atype == 10 {
                // array_type = DataType::Int;
                4
            } else if atype == 11 {
                // array_type = DataType::Long;
                8
            } else {
                panic!("wrong atype");
            }
        };
        let mut size = ARRAY_HEADER_SIZE_BASIC + size * len;
        if size <= 8 {
            size = 8;
        } else {
            size = ((size + 7) / 8) * 8;
        }
        let object_id = self.malloc(size);
        let start = self.address_map[object_id] as usize;

        //设置数组长度
        let lena = u8c::split_u32_to_u8(len);
        self.memory[start + 2] = lena[0];
        self.memory[start + 3] = lena[1];
        self.memory[start + 4] = lena[2];
        self.memory[start + 5] = lena[3];

        //设置数组维度
        self.memory[start + 6] = dimension;

        //self.memory[start] = 0b10000000;
        //设置数组类型
        self.memory[start + 7] = atype;

        object_id
    }

    /**
     * 创建引用类型数组对象
     * 对象头（2个字节，第1位 0,第2位 1 如果atype != 12 第3位为0否则为1）  + 数组长度4个字节 +  维度 1个字节   + class_id 4个字节  + 数组数据 + 对齐
     */
    pub fn create_reference_array(
        &mut self,
        class_id: u32,
        len: u32,
        dimension: u8,
        atype: u8,
    ) -> usize {
        let start_size = ARRAY_HEADER_SIZE_REFERENCE;
        let mut size = start_size + 4 * len;

        // 确保size是8的倍数
        if size < 8 {
            size = 8;
        } else {
            size = ((size + 7) / 8) * 8;
        }

        let object_id = self.malloc(size);
        let start = self.address_map[object_id] as usize;

        //第二位必须是1 ，用于区分基本类型数组和引用类型数组
        //如果为引用类型数组第3位为1，如果为基本类型的多维数组，第3位为0
        if atype == 12 {
            self.memory[start] = REFERENCE_ARRAY_FLAG;
        } else {
            self.memory[start] = REFERENCE_ARRAY_MULTI_FLAG;
        }
       // info!("####{:#b}####", self.memory[start]);

        let lena = u8c::split_u32_to_u8(len);
        self.memory[start + 2] = lena[0];
        self.memory[start + 3] = lena[1];
        self.memory[start + 4] = lena[2];
        self.memory[start + 5] = lena[3];

        self.memory[start + 6] = dimension;

        //如果为基本类型的多维数组这里暂时不设置
        if atype == 12 {
            let cid = u8c::split_u32_to_u8(class_id);
            self.memory[start + 7] = cid[0];
            self.memory[start + 8] = cid[1];
            self.memory[start + 9] = cid[2];
            self.memory[start + 10] = cid[3];
        }else{
            self.memory[start + 7] = atype;
        }

        object_id
    }

    /**
     * 设置基本类型数组元素
     */
    pub fn put_basic_array_element(&mut self, reference_id: u32, index: usize, value: u64) {
        let start_index = self.address_map[reference_id as usize] as usize;
        let atype = self.memory[start_index + 7];
        let offset = 8;
        if atype == 4 {
            // array_type = DataType::Boolean;
            //1
            self.memory[start_index + offset + index] = value as u8;
        } else if atype == 5 {
            // array_type = DataType::Char;
            //2
            let value = value as u16;
            let array = u8c::split_u16_to_u8(value);
            let index = index * 2;
            self.memory[start_index + offset + index] = array[0];
            self.memory[start_index + offset + index + 1] = array[1];
        } else if atype == 6 {
            // array_type = DataType::Float;
            //4
            let value: u32 = value as u32;
            let array = u8c::split_u32_to_u8(value);
            let index = index * 4;
            self.memory[start_index + offset + index] = array[0];
            self.memory[start_index + offset + index + 1] = array[1];
            self.memory[start_index + offset + index + 2] = array[2];
            self.memory[start_index + offset + index + 3] = array[3];
        } else if atype == 7 {
            //  array_type = DataType::Double;
            // 8
            let array = u8c::split_u64_to_u8(value);
            let index = index * 8;
            self.memory[start_index + offset + index] = array[0];
            self.memory[start_index + offset + index + 1] = array[1];
            self.memory[start_index + offset + index + 2] = array[2];
            self.memory[start_index + offset + index + 3] = array[3];
            self.memory[start_index + offset + index + 4] = array[4];
            self.memory[start_index + offset + index + 5] = array[5];
            self.memory[start_index + offset + index + 6] = array[6];
            self.memory[start_index + offset + index + 7] = array[7];
        } else if atype == 8 {
            //  array_type = DataType::Byte;
            // 1
            self.memory[start_index + offset + index] = value as u8;
        } else if atype == 9 {
            // array_type = DataType::Short;
            // 2
            let value = value as u16;
            let index = index * 2;
            let array = u8c::split_u16_to_u8(value);
            self.memory[start_index + offset + index] = array[0];
            self.memory[start_index + offset + index + 1] = array[1];
        } else if atype == 10 {
            // array_type = DataType::Int;
            // 4
            let value: u32 = value as u32;
            let array = u8c::split_u32_to_u8(value);
            let index = index * 4;
            self.memory[start_index + offset + index] = array[0];
            self.memory[start_index + offset + index + 1] = array[1];
            self.memory[start_index + offset + index + 2] = array[2];
            self.memory[start_index + offset + index + 3] = array[3];
        } else if atype == 11 {
            // array_type = DataType::Long;
            //8
            let array = u8c::split_u64_to_u8(value);
            let index = index * 8;
            self.memory[start_index + offset + index] = array[0];
            self.memory[start_index + offset + index + 1] = array[1];
            self.memory[start_index + offset + index + 2] = array[2];
            self.memory[start_index + offset + index + 3] = array[3];
            self.memory[start_index + offset + index + 4] = array[4];
            self.memory[start_index + offset + index + 5] = array[5];
            self.memory[start_index + offset + index + 6] = array[6];
            self.memory[start_index + offset + index + 7] = array[7];
        } else {
            panic!("wrong atype :{},atype_index:{}", atype, start_index + 2);
        }
    }

    /**
     * 设置引用类型数组元素
     */
    pub fn put_reference_array_element(&mut self, reference_id: u32, index: usize, value: u64) {
        //info!("========{},{},{}=========",reference_id,index,value);
        let start_index = self.address_map[reference_id as usize] as usize;
        let offset = 11;
        let value: u32 = value as u32;
        let array = u8c::split_u32_to_u8(value);
        let index = index * 4;
        self.memory[start_index + offset + index] = array[0];
        self.memory[start_index + offset + index + 1] = array[1];
        self.memory[start_index + offset + index + 2] = array[2];
        self.memory[start_index + offset + index + 3] = array[3];
    }

    /**
     * 设置数组元素
     */
    pub fn put_array_element(&mut self, reference_id: u32, index: usize, value: u64) {
        let start_index = self.address_map[reference_id as usize] as usize;
        let flag = self.memory[start_index] & 0b01000000;
        if flag == 0 {
            self.put_basic_array_element(reference_id, index, value);
        } else {
            self.put_reference_array_element(reference_id, index, value);
        }
    }

    /**
     * 读取数组元素
     */
    pub fn get_array_element(&self, reference_id: u32, index: usize) -> (u8, Option<u64>) {
        let start_index = self.address_map[reference_id as usize] as usize;
        let flag = self.memory[start_index] & 0b01000000;
        if flag == 0 {
            self.get_basic_array_element(reference_id, index)
        } else {
            (12, self.get_reference_array_element(reference_id, index))
        }
    }

    /**
     * 读取数组长度
     */
    pub fn get_array_length(&self, reference_id: u32) -> u32 {
        let start_index = self.address_map[reference_id as usize] as usize;
        let bytes = [
            self.memory[start_index + 2],
            self.memory[start_index + 3],
            self.memory[start_index + 4],
            self.memory[start_index + 5],
        ];
        u32::from_be_bytes(bytes)
    }

    /**
     * 读取数组的atype,dimension,class_id
     */
    pub fn get_array_info(&self, reference_id: u32) -> (Option<u32>,u8, u8) {
        let start_index = self.address_map[reference_id as usize] as usize;
        //0b01100000 为引用类型数组
        //0b00000000 为基本类型数组
        //0b01000000 基本类型多维数组
        //info!("==={:08b}====", self.memory[start_index]);
       if self.memory[start_index] & 0b00100000 != 0 {
            let cid = [
                self.memory[start_index + 7],
                self.memory[start_index + 8],
                self.memory[start_index + 9],
                self.memory[start_index + 10],
            ];
            (Some(u32::from_be_bytes(cid)),12,self.memory[start_index + 6])
         }else if self.memory[start_index] & 0b01000000 != 0 {
            (None,self.memory[start_index + 7],self.memory[start_index + 6])
         }else{
            (None,self.memory[start_index + 7],self.memory[start_index + 6])
         }
    }

    /**
     * 读取基本类型数组元素
     */
    pub fn get_basic_array_element(&self, reference_id: u32, index: usize) -> (u8, Option<u64>) {
        let start_index = self.address_map[reference_id as usize] as usize;
        let atype = self.memory[start_index + 7];
        let offset = 8;
        match atype {
            4 | 8 => {
                // Boolean or Byte (1 byte)
                let value = self.memory[start_index + offset + index] as u64;
                (atype, Some(value))
            }
            5 | 9 => {
                // Char or Short (2 bytes)
                let start = start_index + offset + index * 2;
                let bytes = [self.memory[start], self.memory[start + 1]];
                let value = u16::from_be_bytes(bytes) as u64;
                (atype, Some(value))
            }
            6 | 10 => {
                // Float or Int (4 bytes)
                let start = start_index + offset + index * 4;
                let bytes = [
                    self.memory[start],
                    self.memory[start + 1],
                    self.memory[start + 2],
                    self.memory[start + 3],
                ];
                let value = u32::from_be_bytes(bytes) as u64;
                (atype, Some(value))
            }
            7 | 11 => {
                // Double or Long (8 bytes)
                let start = start_index + offset + index * 8;
                let bytes = [
                    self.memory[start],
                    self.memory[start + 1],
                    self.memory[start + 2],
                    self.memory[start + 3],
                    self.memory[start + 4],
                    self.memory[start + 5],
                    self.memory[start + 6],
                    self.memory[start + 7],
                ];
                let value = u64::from_be_bytes(bytes);
                (atype, Some(value))
            }
            _ => {
                panic!("wrong atype: {}", atype);
            }
        }
    }

    /**
     * 获取引用类型数组元素（使用大端字节序）
     */
    pub fn get_reference_array_element(&self, reference_id: u32, index: usize) -> Option<u64> {
        let start_index = self.address_map[reference_id as usize] as usize;
        let offset = 11;
        let base_index = start_index + offset + (index * 4);

        // 创建字节数组并读取数据
        let bytes = [
            self.memory[base_index],
            self.memory[base_index + 1],
            self.memory[base_index + 2],
            self.memory[base_index + 3],
        ];

         if bytes == [0, 0, 0, 0] {
            return None;
        }
        // 使用大端字节序转换为u32，再转为u64
        Some(u32::from_be_bytes(bytes) as u64)
    }

    pub fn get_field_i32(&self, reference_id: u32, offset: u32) -> i32 {
        // 参数直接传递值而不是引用，减少解引用开销
        let start_index = (self.address_map[reference_id as usize] + 6 + offset) as usize;

        // 直接使用数组索引而不是slice创建，减少中间步骤
        let bytes = [
            self.memory[start_index],
            self.memory[start_index + 1],
            self.memory[start_index + 2],
            self.memory[start_index + 3],
        ];

        i32::from_be_bytes(bytes)
    }

    pub fn get_field_ptr(&self, reference_id: u32, offset: u32) -> Option<u32> {
        let start_index = (self.address_map[reference_id as usize] + 6 + offset) as usize;
        let bytes = [
            self.memory[start_index],
            self.memory[start_index + 1],
            self.memory[start_index + 2],
            self.memory[start_index + 3],
        ];
        if bytes == [0, 0, 0, 0] {
            return None;
        }
        Some(u32::from_be_bytes(bytes))
    }

    pub fn get_field_i8(&self, reference_id: u32, offset: u32) -> i8 {
        let start_index = (self.address_map[reference_id as usize] + 6 + offset) as usize;
        i8::from_be_bytes([self.memory[start_index]])
    }

    pub fn get_field_i16(&self, reference_id: u32, offset: u32) -> i16 {
        let start_index = (self.address_map[reference_id as usize] + 6 + offset) as usize;
        let bytes = [self.memory[start_index], self.memory[start_index + 1]];
        i16::from_be_bytes(bytes)
    }

    pub fn get_field_i64(&self, reference_id: u32, offset: u32) -> i64 {
        let start_index = (self.address_map[reference_id as usize] + 6 + offset) as usize;
        let bytes = [
            self.memory[start_index],
            self.memory[start_index + 1],
            self.memory[start_index + 2],
            self.memory[start_index + 3],
            self.memory[start_index + 4],
            self.memory[start_index + 5],
            self.memory[start_index + 6],
            self.memory[start_index + 7],
        ];
        i64::from_be_bytes(bytes)
    }

    pub fn get_field_f32(&self, reference_id: u32, offset: u32) -> f32 {
        let start_index = (self.address_map[reference_id as usize] + 6 + offset) as usize;
        let bytes = [
            self.memory[start_index],
            self.memory[start_index + 1],
            self.memory[start_index + 2],
            self.memory[start_index + 3],
        ];
        f32::from_be_bytes(bytes)
    }

    pub fn get_field_f64(&self, reference_id: u32, offset: u32) -> f64 {
        let start_index = (self.address_map[reference_id as usize] + 6 + offset) as usize;
        let bytes = [
            self.memory[start_index],
            self.memory[start_index + 1],
            self.memory[start_index + 2],
            self.memory[start_index + 3],
            self.memory[start_index + 4],
            self.memory[start_index + 5],
            self.memory[start_index + 6],
            self.memory[start_index + 7],
        ];
        f64::from_be_bytes(bytes)
    }

    pub fn get_class(&self, reference_id: u32) -> u32 {
        let start_index = (self.address_map[reference_id as usize] + 2 as u32) as usize;
        let bytes = [
            self.memory[start_index],
            self.memory[start_index + 1],
            self.memory[start_index + 2],
            self.memory[start_index + 3],
        ];
        u32::from_be_bytes(bytes)
    }

    pub fn put_field_i64(&mut self, reference_id: u32, offset: u32, value: i64) {
        let array = u8c::split_i64_to_u8(value);
        let start_index = (self.address_map[reference_id as usize] + 6 + offset) as usize;
        self.memory[start_index] = array[0];
        self.memory[start_index + 1] = array[1];
        self.memory[start_index + 2] = array[2];
        self.memory[start_index + 3] = array[3];
        self.memory[start_index + 4] = array[4];
        self.memory[start_index + 5] = array[5];
        self.memory[start_index + 6] = array[6];
        self.memory[start_index + 7] = array[7];
    }

    pub fn put_field_i32(&mut self, reference_id: u32, offset: u32, value: i32) {
        let array = u8c::split_i32_to_u8(value);
        let start_index = (self.address_map[reference_id as usize] + 6 + offset) as usize;
        self.memory[start_index] = array[0];
        self.memory[start_index + 1] = array[1];
        self.memory[start_index + 2] = array[2];
        self.memory[start_index + 3] = array[3];
    }

    pub fn put_field_f32(&mut self, reference_id: u32, offset: u32, value: f32) {
        let array = value.to_be_bytes();
        let start_index = (self.address_map[reference_id as usize] + 6 + offset) as usize;
        self.memory[start_index] = array[0];
        self.memory[start_index + 1] = array[1];
        self.memory[start_index + 2] = array[2];
        self.memory[start_index + 3] = array[3];
    }

    pub fn put_field_f64(&mut self, reference_id: u32, offset: u32, value: f64) {
        let array = value.to_be_bytes();
        let start_index = (self.address_map[reference_id as usize] + 6 + offset) as usize;
        self.memory[start_index] = array[0];
        self.memory[start_index + 1] = array[1];
        self.memory[start_index + 2] = array[2];
        self.memory[start_index + 3] = array[3];
        self.memory[start_index + 4] = array[4];
        self.memory[start_index + 5] = array[5];
        self.memory[start_index + 6] = array[6];
        self.memory[start_index + 7] = array[7];
    }

    pub fn put_field_reference(&mut self, reference_id: u32, offset: u32, value: u32) {
        let array = u8c::split_u32_to_u8(value as u32);
        let start_index = (self.address_map[reference_id as usize] + 6 + offset) as usize;
        self.memory[start_index] = array[0];
        self.memory[start_index + 1] = array[1];
        self.memory[start_index + 2] = array[2];
        self.memory[start_index + 3] = array[3];
    }

    pub fn put_field_u32(&mut self, reference_id: u32, offset: u32, value: u32) {
        let array = u8c::split_i32_to_u8(value as i32);
        let start_index = (self.address_map[reference_id as usize] + 6 + offset) as usize;
        self.memory[start_index] = array[0];
        self.memory[start_index + 1] = array[1];
        self.memory[start_index + 2] = array[2];
        self.memory[start_index + 3] = array[3];
    }

    pub fn put_field_i16(&mut self, reference_id: u32, offset: u32, value: i16) {
        let array = u8c::split_i16_to_u8(value);
        let start_index = (self.address_map[reference_id as usize] + 6 + offset) as usize;
        self.memory[start_index] = array[0];
        self.memory[start_index + 1] = array[1];
    }

    pub fn put_field_i8(&mut self, reference_id: u32, offset: u32, value: i8) {
        let start_index = (self.address_map[reference_id as usize] + 6 + offset) as usize;
        self.memory[start_index] = value as u8
    }

    //get_constant_pool_class
    pub fn get_constant_pool_class(&self, class_name: &String) -> Option<&u32> {
        self.class_pool.get(class_name)
    }

    pub fn put_into_class_constant_pool(&mut self, class_name: String, class_object_id: u32) {
        self.class_pool.insert(class_name, class_object_id);
    }

    pub fn get_constant_string_pool(&self, string: &String) -> Option<&u32> {
        self.str_pool.get(string)
    }
    pub fn put_into_string_constant_pool(&mut self, string: String, string_id: u32) {
        self.str_pool.insert(string, string_id);
    }
}
