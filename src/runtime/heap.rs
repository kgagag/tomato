use std::collections::HashMap;

use log::info;
use crate::{
    classfile::class::Class,
    common::{error::Throwable, param::DataType},
    utils::u8c,
};

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

// atype 常量定义
const ATYPE_BOOLEAN: u8 = 4;
const ATYPE_CHAR: u8 = 5;
const ATYPE_FLOAT: u8 = 6;
const ATYPE_DOUBLE: u8 = 7;
const ATYPE_BYTE: u8 = 8;
const ATYPE_SHORT: u8 = 9;
const ATYPE_INT: u8 = 10;
const ATYPE_LONG: u8 = 11;
const ATYPE_REFERENCE: u8 = 12;


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
            //省略0这个位置
            address_map_index: 1,
            address_malloc_method: 0,
            str_pool: HashMap::new(),
            class_pool: HashMap::new(),
        }
    }

    /// 计算对齐后的大小（8字节对齐）
    #[inline]
    fn align_size(size: u32) -> u32 {
        if size < 8 {
            8
        } else {
            ((size + 7) / 8) * 8
        }
    }

    /// 获取对象在内存中的起始地址
    #[inline]
    fn get_object_address(&self, object_id: usize) -> usize {
        self.address_map[object_id] as usize
    }

    /// 获取数组元素类型的大小
    #[inline]
    fn get_atype_size(atype: u8) -> u32 {
        match atype {
            ATYPE_BOOLEAN | ATYPE_BYTE => 1,
            ATYPE_CHAR | ATYPE_SHORT => 2,
            ATYPE_FLOAT | ATYPE_INT => 4,
            ATYPE_DOUBLE | ATYPE_LONG => 8,
            _ => panic!("wrong atype: {}", atype),
        }
    }

    /**
     * 分配内存,如果内存块足够，则分配成功,更新内存块信息
     * 返回对象id
     */
    fn malloc(&mut self, size: u32) -> Result<usize, Throwable> {
        // 查找合适的内存块（使用最佳适配策略）
        let mut best_index = None;
        let mut best_size = u32::MAX;

        for (i, (_, block_size)) in self.memory_block.iter().enumerate() {
            if *block_size >= size && *block_size < best_size {
                best_index = Some(i);
                best_size = *block_size;
            }
        }

        let delete_index = match best_index {
            Some(idx) => idx,
            None => return Err(Throwable::Error(crate::common::error::JvmError::OutOfMemoryError)),
        };

        let (address, block_size) = self.memory_block[delete_index];

        // 分割剩余内存块
        let new_block_size = block_size - size;
        if new_block_size > 0 {
            let new_address = address + size;
            self.memory_block.push((new_address, new_block_size));
        }

        // 使用 swap_remove 替代 remove，避免 O(n) 的数据移动
        self.memory_block.swap_remove(delete_index);

        // 更新address_map
        let index = self.address_map_index;
        self.address_map[index] = address;

        // 更新address_map_index
        self.update_address_map_index();
        Ok(index)
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
        // 如果没有找到空闲位置，扩展 address_map
        let old_len = self.address_map.len();
        self.address_map.resize(old_len * 2, 0);
        self.address_map_index = old_len;
    }

    //非数组
    // 对象头（2个字节，第1位 1） class_id（4个字节）+ 对象数据 + 对齐
    pub fn create_object(&mut self, class: &Class) -> Result<usize, Throwable> {
        let mut size: u32 = OBJECT_HEADER_SIZE;
        for (_key, value) in &class.field_info {
            size += match &value.data_type {
                DataType::Byte | DataType::Boolean => 1,
                DataType::Char | DataType::Short => 2,
                DataType::Float | DataType::Int => 4,
                DataType::Double | DataType::Long => 8,
                DataType::Reference(_) | DataType::Array { .. } => 4,
                _ => panic!("unsupported data type"),
            };
        }

        size = Self::align_size(size);
        let object_id = self.malloc(size)?;
        let start = self.get_object_address(object_id);

        self.memory[start] = OBJECT_FLAG;
        // 设置class_id
        let cid = u8c::split_u32_to_u8(class.id as u32);
        self.memory[start + 2..start + 6].copy_from_slice(&cid);

        Ok(object_id)
    }


   /**
    * 检查一个对象是否是数组
    */
    pub fn is_array(&self, object_id: usize) -> bool {
        let start = self.address_map[object_id] as usize;
        self.memory[start] & 0b10000000 == 0
    }

    pub fn get_object_class_id(&self, object_id: usize) -> Result<u32, Throwable> {
        let start = self.address_map[object_id] as usize;
        let bytes = [
            self.memory[start + 2],
            self.memory[start + 3],
            self.memory[start + 4],
            self.memory[start + 5],
        ];
        Ok(u8c::combine_u8_to_u32(bytes))
    }


    /**
     * 创建基本类型数组对象
     * 对象头（2个字节，第1位 0,第 2 位 0）+ 数组长度 4 +维度 1个字 + data_type 1个字节 + 数组数据 + 对齐
     */
    pub fn create_basic_array(&mut self, atype: u8, len: u32, dimension: u8) -> Result<usize, Throwable> {
        let elem_size = Self::get_atype_size(atype);
        let size = Self::align_size(ARRAY_HEADER_SIZE_BASIC + elem_size * len);

        let object_id = self.malloc(size)?;
        let start = self.get_object_address(object_id);

        //设置数组长度
        let len_bytes = len.to_be_bytes();
        self.memory[start + 2..start + 6].copy_from_slice(&len_bytes);

        //设置数组维度
        self.memory[start + 6] = dimension;

        //设置数组类型
        self.memory[start + 7] = atype;

        Ok(object_id)
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
    ) -> Result<usize, Throwable> {
        let size = Self::align_size(ARRAY_HEADER_SIZE_REFERENCE + 4 * len);

        let object_id = self.malloc(size)?;
        let start = self.get_object_address(object_id);

        //第二位必须是1 ，用于区分基本类型数组和引用类型数组
        //如果为引用类型数组第3位为1，如果为基本类型的多维数组，第3位为0
        self.memory[start] = if atype == ATYPE_REFERENCE {
            REFERENCE_ARRAY_FLAG
        } else {
            REFERENCE_ARRAY_MULTI_FLAG
        };

        //设置数组长度
        let len_bytes = len.to_be_bytes();
        self.memory[start + 2..start + 6].copy_from_slice(&len_bytes);

        self.memory[start + 6] = dimension;

        //如果为基本类型的多维数组这里暂时不设置
        if atype == ATYPE_REFERENCE {
            let cid = u8c::split_u32_to_u8(class_id);
            self.memory[start + 7..start + 11].copy_from_slice(&cid);
        } else {
            self.memory[start + 7] = atype;
        }

        Ok(object_id)
    }

    /**
     * 设置基本类型数组元素
     */
    pub fn put_basic_array_element(&mut self, reference_id: u32, index: usize, value: u64) {
        let start_index = self.address_map[reference_id as usize] as usize;
        let atype = self.memory[start_index + 7];
        let offset = 8;

        match atype {
            ATYPE_BOOLEAN | ATYPE_BYTE => {
                self.memory[start_index + offset + index] = value as u8;
            }
            ATYPE_CHAR | ATYPE_SHORT => {
                let bytes = (value as u16).to_be_bytes();
                let idx = index * 2;
                self.memory[start_index + offset + idx..start_index + offset + idx + 2]
                    .copy_from_slice(&bytes);
            }
            ATYPE_FLOAT | ATYPE_INT => {
                let bytes = (value as u32).to_be_bytes();
                let idx = index * 4;
                self.memory[start_index + offset + idx..start_index + offset + idx + 4]
                    .copy_from_slice(&bytes);
            }
            ATYPE_DOUBLE | ATYPE_LONG => {
                let bytes = value.to_be_bytes();
                let idx = index * 8;
                self.memory[start_index + offset + idx..start_index + offset + idx + 8]
                    .copy_from_slice(&bytes);
            }
            _ => panic!("wrong atype: {}", atype),
        }
    }

    /**
     * 设置引用类型数组元素
     */
    pub fn put_reference_array_element(&mut self, reference_id: u32, index: usize, value: u64) {
        let start_index = self.address_map[reference_id as usize] as usize;
        let offset = 11;
        let bytes = (value as u32).to_be_bytes();
        let idx = index * 4;
        self.memory[start_index + offset + idx..start_index + offset + idx + 4]
            .copy_from_slice(&bytes);
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
            (ATYPE_REFERENCE, self.get_reference_array_element(reference_id, index))
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
    pub fn get_array_info(&self, reference_id: u32) -> (Option<u32>, u8, u8) {
        let start_index = self.address_map[reference_id as usize] as usize;

        if self.memory[start_index] & 0b00100000 != 0 {
            let bytes = [
                self.memory[start_index + 7],
                self.memory[start_index + 8],
                self.memory[start_index + 9],
                self.memory[start_index + 10],
            ];
            (Some(u32::from_be_bytes(bytes)), ATYPE_REFERENCE, self.memory[start_index + 6])
        } else if self.memory[start_index] & 0b01000000 != 0 {
            (None, self.memory[start_index + 7], self.memory[start_index + 6])
        } else {
            (None, self.memory[start_index + 7], self.memory[start_index + 6])
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
            ATYPE_BOOLEAN | ATYPE_BYTE => {
                let value = self.memory[start_index + offset + index] as u64;
                (atype, Some(value))
            }
            ATYPE_CHAR | ATYPE_SHORT => {
                let start = start_index + offset + index * 2;
                let bytes = [self.memory[start], self.memory[start + 1]];
                let value = u16::from_be_bytes(bytes) as u64;
                (atype, Some(value))
            }
            ATYPE_FLOAT | ATYPE_INT => {
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
            ATYPE_DOUBLE | ATYPE_LONG => {
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

        let bytes = [
            self.memory[base_index],
            self.memory[base_index + 1],
            self.memory[base_index + 2],
            self.memory[base_index + 3],
        ];

        if bytes == [0, 0, 0, 0] {
            None
        } else {
            Some(u32::from_be_bytes(bytes) as u64)
        }
    }

    #[inline]
    fn get_field_start_index(&self, reference_id: u32, offset: u32) -> usize {
        let base = self.address_map[reference_id as usize] as usize;
        base + 6 + offset as usize
    }

    pub fn get_field_i32(&self, reference_id: u32, offset: u32) -> i32 {
        let start_index = self.get_field_start_index(reference_id, offset);
        let bytes = [
            self.memory[start_index],
            self.memory[start_index + 1],
            self.memory[start_index + 2],
            self.memory[start_index + 3],
        ];
        i32::from_be_bytes(bytes)
    }

    pub fn get_field_ptr(&self, reference_id: u32, offset: u32) -> Option<u32> {
        let start_index = self.get_field_start_index(reference_id, offset);
        let bytes = [
            self.memory[start_index],
            self.memory[start_index + 1],
            self.memory[start_index + 2],
            self.memory[start_index + 3],
        ];
        if bytes == [0, 0, 0, 0] {
            None
        } else {
            Some(u32::from_be_bytes(bytes))
        }
    }

    pub fn get_field_i8(&self, reference_id: u32, offset: u32) -> i8 {
        let start_index = self.get_field_start_index(reference_id, offset);
        i8::from_be_bytes([self.memory[start_index]])
    }

    pub fn get_field_i16(&self, reference_id: u32, offset: u32) -> i16 {
        let start_index = self.get_field_start_index(reference_id, offset);
        let bytes = [self.memory[start_index], self.memory[start_index + 1]];
        i16::from_be_bytes(bytes)
    }

    pub fn get_field_i64(&self, reference_id: u32, offset: u32) -> i64 {
        let start_index = self.get_field_start_index(reference_id, offset);
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
        let start_index = self.get_field_start_index(reference_id, offset);
        let bytes = [
            self.memory[start_index],
            self.memory[start_index + 1],
            self.memory[start_index + 2],
            self.memory[start_index + 3],
        ];
        f32::from_be_bytes(bytes)
    }

    pub fn get_field_f64(&self, reference_id: u32, offset: u32) -> f64 {
        let start_index = self.get_field_start_index(reference_id, offset);
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
        let start_index = self.address_map[reference_id as usize] as usize + 2;
        let bytes = [
            self.memory[start_index],
            self.memory[start_index + 1],
            self.memory[start_index + 2],
            self.memory[start_index + 3],
        ];
        u32::from_be_bytes(bytes)
    }

    pub fn put_field_i64(&mut self, reference_id: u32, offset: u32, value: i64) {
        let start_index = self.get_field_start_index(reference_id, offset);
        let bytes = value.to_be_bytes();
        self.memory[start_index..start_index + 8].copy_from_slice(&bytes);
    }

    pub fn put_field_i32(&mut self, reference_id: u32, offset: u32, value: i32) {
        let start_index = self.get_field_start_index(reference_id, offset);
        let bytes = value.to_be_bytes();
        self.memory[start_index..start_index + 4].copy_from_slice(&bytes);
    }

    pub fn put_field_f32(&mut self, reference_id: u32, offset: u32, value: f32) {
        let start_index = self.get_field_start_index(reference_id, offset);
        let bytes = value.to_be_bytes();
        self.memory[start_index..start_index + 4].copy_from_slice(&bytes);
    }

    pub fn put_field_f64(&mut self, reference_id: u32, offset: u32, value: f64) {
        let start_index = self.get_field_start_index(reference_id, offset);
        let bytes = value.to_be_bytes();
        self.memory[start_index..start_index + 8].copy_from_slice(&bytes);
    }

    pub fn put_field_reference(&mut self, reference_id: u32, offset: u32, value: u32) {
        let start_index = self.get_field_start_index(reference_id, offset);
        let bytes = value.to_be_bytes();
        self.memory[start_index..start_index + 4].copy_from_slice(&bytes);
    }

    pub fn put_field_u32(&mut self, reference_id: u32, offset: u32, value: u32) {
        let start_index = self.get_field_start_index(reference_id, offset);
        let bytes = value.to_be_bytes();
        self.memory[start_index..start_index + 4].copy_from_slice(&bytes);
    }

    pub fn put_field_i16(&mut self, reference_id: u32, offset: u32, value: i16) {
        let start_index = self.get_field_start_index(reference_id, offset);
        let bytes = value.to_be_bytes();
        self.memory[start_index..start_index + 2].copy_from_slice(&bytes);
    }

    pub fn put_field_i8(&mut self, reference_id: u32, offset: u32, value: i8) {
        let start_index = self.get_field_start_index(reference_id, offset);
        self.memory[start_index] = value as u8;
    }

    pub fn get_constant_pool_class(&self, class_name: &str) -> Option<u32> {
        self.class_pool.get(class_name).copied()
    }

    pub fn put_into_class_constant_pool(&mut self, class_name: String, class_object_id: u32) {
        self.class_pool.insert(class_name, class_object_id);
    }

    pub fn get_constant_string_pool(&self, string: &str) -> Option<u32> {
        self.str_pool.get(string).copied()
    }

    pub fn put_into_string_constant_pool(&mut self, string: String, string_id: u32) {
        self.str_pool.insert(string, string_id);
    }
}
