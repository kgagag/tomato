use std::collections::HashMap;

use crate::common::{param::DataType, value::StackFrameValue};

#[derive(Debug, Clone)]
pub struct Class {
    pub magic: u32,
    pub minor_version: u16,
    pub major_version: u16,
    pub constant_pool_count: u16,
    pub constant_pool: HashMap<u16, ConstantPoolInfo>,
    pub access_flags: u16,
    pub this_class: u16,
    pub super_class: u16,
    pub interface_count: u16,
    pub interfaces: Vec<u16>,
    pub fields_count: u16,
    pub field_info: HashMap<String,FieldInfo>,
    pub methods_count: u16,
    pub method_info: Vec<MethodInfo>,
    pub attributes_count: u16,
    pub attribute_info: Vec<AttributeInfo>,
    pub id: usize,
    pub class_name: String,
    pub super_class_name:String
}

impl Class {
    pub fn new() -> Self {
        Self {
            magic: 0,
            minor_version: 0,
            major_version: 0,
            constant_pool_count: 0,
            constant_pool: HashMap::new(),
            access_flags: 0,
            this_class: 0,
            super_class: 0,
            interface_count: 0,
            interfaces: Vec::new(),
            fields_count: 0,
            field_info: HashMap::new(),
            methods_count: 0,
            method_info: Vec::new(),
            attributes_count: 0,
            attribute_info: Vec::new(),
            id: 0,
            class_name: String::new(),
            super_class_name:String::new(),
        }
    }
}

// 定义 AccessFlag 枚举，每个变量代表不同的访问标志
#[derive(Debug, Clone, Copy)]
#[repr(u16)]
pub enum FieldAccessFlag {
    Public = 0x0001,
    Private = 0x0002,
    Protected = 0x0004,
    Static = 0x0008,
    Final = 0x0010,
    Volatile = 0x0040,
    Transient = 0x0080,
    Synthetic = 0x1000,
    Enum = 0x4000,
}

impl FieldAccessFlag {
    pub fn is_set(self, flags: u16) -> bool {
        flags & (self as u16) != 0
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u16)]
pub enum MethodAccessFlag {
    Public = 0x0001,
    Private = 0x0002,
    Protected = 0x0004,
    Static = 0x0008,
    Final = 0x0010,
    Synchronized = 0x0020,
    Bridge = 0x0040,
    Varargs = 0x0080,
    Native = 0x0100,
    Abstract = 0x0400,
    Strict = 0x0800,
    Synthetic = 0x1000,
}

impl MethodAccessFlag {
    pub fn is_set(self, flags: u16) -> bool {
        flags & (self as u16) != 0
    }
}

#[derive(Debug, Clone)]
pub struct MethodInfo {
    pub access_flag: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes_count: u16,
    pub param: Vec<DataType>,
    pub attributes: Vec<AttributeInfo>,
    pub class_name: String,
    pub class_id:u32,
    pub method_name: String,
    pub descriptor: String,
}

#[derive(Debug, Clone)]
pub struct FieldInfo {
    pub access_flag: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attribute_count: u16,
    pub atrributes: Vec<AttributeInfo>,
    pub field_name: String,
    pub value: StackFrameValue,
    pub data_type: DataType,
    pub descriptor: String,
    pub field_index:u16
}

#[derive(Debug, Clone)]
pub enum AttributeInfo {
    Code(CodeAttribute),
}

#[derive(Debug, Clone)]
pub struct CodeAttribute {
    pub max_stack: u16,
    pub max_locals: u16,
    pub code_length: u32,
    pub code: Vec<u8>,
    pub exception_table_length: u16,
    pub exception_table: Vec<ExceptionTable>,
}

impl CodeAttribute {
    pub fn new(
        max_stack: u16,
        max_locals: u16,
        code_length: u32,
        code: Vec<u8>,
        exception_table_length: u16,
        exception_table: Vec<ExceptionTable>,
    ) -> CodeAttribute {
        CodeAttribute {
            max_stack,
            max_locals,
            code_length,
            code,
            exception_table_length,
            exception_table,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ExceptionTable {
    pub start_pc: u16,
    pub end_pc: u16,
    pub handler_pc: u16,
    pub catch_type: u16,
}

impl ExceptionTable {
    pub fn new(start_pc: u16, end_pc: u16, handler_pc: u16, catch_type: u16) -> ExceptionTable {
        ExceptionTable {
            start_pc,
            end_pc,
            handler_pc,
            catch_type,
        }
    }
}

#[derive(Debug)]
pub struct Exception {}

#[derive(Debug, Clone)]
pub enum ConstantPoolInfo {
    Utf8(String),
    Integer(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    Class(u16),
    String(u16),
    Fieldref(u16, u16),
    Methodref(u16, u16),
    InterfaceMethodref(u16, u16),
    NameAndType(u16, u16),
    MethodHandle(u8, u16),
    MethodType(u16),
    InvokeDynamic(u16, u16),
    Module(u16),
    Package(u16),
    MethodPointer(u8, u16),
    InvokeStaticDynamic(u16, u16),
    BootstrapMethod(u16, u16),
    MethodTypeReference(u16),
    // 添加更多可能的常量类型
}
