use crate::classfile::class;
use crate::classfile::class::AttributeInfo;
use crate::classfile::class::Class;
use crate::classfile::class::CodeAttribute;
use crate::classfile::class::ConstantPoolInfo;
use crate::classfile::class::ExceptionTable;
use crate::classfile::class::FieldInfo;
use crate::classfile::class::MethodInfo;
use crate::common::error::JvmError;
use crate::common::error::Throwable;
use crate::common::param::DataType;
use crate::common::stack_frame::*;
use crate::common::value::StackFrameValue;
use crate::interpreter::instructions::op_code::op_code::do_opcode;
use crate::runtime::heap::Heap;
use crate::runtime::metaspace::Metaspace;
use crate::utils::u8c;
use crate::utils::u8c::u8s_to_u16;
use crate::utils::u8c::u8s_to_u32;
use byteorder::{BigEndian, ReadBytesExt};
use linked_hash_map::LinkedHashMap;
use log::info;
use log::warn;
use std::collections::HashMap;
use std::env;
use std::fs::{self, File};
use std::io;
use std::io::BufReader;
use std::io::Cursor;
use std::io::Read;
use std::path::Path;
use std::str::Utf8Error;
use std::sync::Mutex;
use std::time::Instant;
use zip::read::ZipArchive;
fn parse_descriptor(descriptor: &Vec<u8>) -> Result<Option<Vec<DataType>>, String> {
    let mut index = 0;
    let descriptor_length = descriptor.len();
    let mut parameters: Vec<DataType> = Vec::new();
    while index < descriptor_length {
        let descriptor_char = descriptor[index] as char;
        if descriptor_char == '(' {
            index += 1;
            continue;
        }
        if descriptor_char == ')' {
            break;
        }
        //info!("descriptor_char:{:?}", &descriptor_char);
        match descriptor_char {
            'B' => parameters.push(DataType::Byte),
            'C' => parameters.push(DataType::Char),
            'D' => parameters.push(DataType::Double),
            'F' => parameters.push(DataType::Float),
            'I' => parameters.push(DataType::Int),
            'J' => parameters.push(DataType::Long),
            'L' => {
                // Handle reference type parameters
                let mut class_name = String::new();
                while index < descriptor_length {
                    index += 1;
                    if descriptor[index] as char == ';' {
                        break;
                    }
                    class_name.push(descriptor[index] as char);
                }
                parameters.push(DataType::Reference(class_name));
            }
            'S' => parameters.push(DataType::Short),
            'Z' => parameters.push(DataType::Boolean),
            '[' => {
                // Handle array type parameters
                let mut array_depth = 1;
                while index + 1 < descriptor_length && descriptor[index + 1] as char == '[' {
                    array_depth += 1;
                    index += 1;
                }
                index = index + 1;
                let element_type = match descriptor[index] as char {
                    'B' => DataType::Byte,
                    'C' => DataType::Char,
                    'D' => DataType::Double,
                    'F' => DataType::Float,
                    'I' => DataType::Int,
                    'J' => DataType::Long,
                    'L' => {
                        let mut class_name = String::new();
                        while index < descriptor_length {
                            index += 1;
                            if descriptor[index] as char == ';' {
                                break;
                            }
                            class_name.push(descriptor[index] as char);
                        }
                        DataType::Reference(class_name)
                    }
                    'S' => DataType::Short,
                    'Z' => DataType::Boolean,
                    _ => {
                        warn!("Unknown array element type:{:?}", descriptor_char);
                        return Err("Unknown array element type".to_string());
                    }
                };
                parameters.push(DataType::Array {
                    element_type: Box::new(element_type),
                    depth: array_depth,
                });
            }
            _ => {}
        }
        index += 1;
    }
    if parameters.is_empty() {
        Ok(None)
    } else {
        Ok(Some(parameters))
    }
}

fn parse_field_descriptor(descriptor: Vec<u8>) -> DataType {
    let mut index: usize = 0;
    let descriptor_length = descriptor.len();
    let descriptor_char = descriptor[index] as char;
    //info!("descriptor_char:{:?}", &descriptor_char);
    match descriptor_char {
        'B' => DataType::Byte,
        'C' => DataType::Char,
        'D' => DataType::Double,
        'F' => DataType::Float,
        'I' => DataType::Int,
        'J' => DataType::Long,
        'L' => {
            // Handle reference type parameters
            let mut class_name = String::new();
            while index < descriptor_length {
                index += 1;
                if descriptor[index] as char == ';' {
                    break;
                }
                class_name.push(descriptor[index] as char);
            }
            DataType::Reference(class_name)
        }
        'S' => DataType::Short,
        'Z' => DataType::Boolean,
        '[' => {
            // Handle array type parameters
            let mut array_depth = 1;
            while index + 1 < descriptor_length && descriptor[index + 1] as char == '[' {
                array_depth += 1;
                index += 1;
            }
            index = index + 1;
            let element_type = match descriptor[index] as char {
                'B' => DataType::Byte,
                'C' => DataType::Char,
                'D' => DataType::Double,
                'F' => DataType::Float,
                'I' => DataType::Int,
                'J' => DataType::Long,
                'L' => {
                    let mut class_name = String::new();
                    while index < descriptor_length {
                        index += 1;
                        if descriptor[index] as char == ';' {
                            break;
                        }
                        class_name.push(descriptor[index] as char);
                    }
                    DataType::Reference(class_name)
                }
                'S' => DataType::Short,
                'Z' => DataType::Boolean,
                _ => {
                    warn!("Unknown array element type:{:?}", descriptor_char);
                    panic!()
                }
            };
            DataType::Array {
                element_type: Box::new(element_type),
                depth: array_depth,
            }
        }
        _ => panic!(""),
    }
}

thread_local! {
    static ARCHIVE_CACHE: Mutex<HashMap<String, ZipArchive<BufReader<std::fs::File>>>> = Mutex::new(HashMap::new());
}

fn read_class_from_jar(jar_path: &str, class_name: &str) -> Option<Vec<u8>> {
    ARCHIVE_CACHE.with(|cache| {
        let mut cache_lock = cache.lock().unwrap();
        
        // 获取或创建 ZipArchive
        let archive = match cache_lock.entry(jar_path.to_string()) {
            std::collections::hash_map::Entry::Occupied(entry) => entry.into_mut(),
            std::collections::hash_map::Entry::Vacant(entry) => {
                let file = std::fs::File::open(jar_path).ok()?;
                let reader = BufReader::new(file);
                let zip_archive = ZipArchive::new(reader).ok()?;
                entry.insert(zip_archive)
            }
        };
        
        // 读取类文件
        let mut entry = archive.by_name(class_name).ok()?;
        let mut buffer = Vec::new();
        entry.read_to_end(&mut buffer).ok()?;
        Some(buffer)
    })
}


fn get_class_from_disk(name: &String) -> Result<Vec<u8>, Throwable> {
    let mut name_with_ext = name.to_string();
    name_with_ext.push_str(".class");
    
    match env::current_dir() {
        Ok(path) => {
            // 构建路径（默认）
            let mut mod_jre_jar_path = path.join("rt-mod.jar");
            let mut rt_jar_path = path.join("rt.jar");
            
            //为了兼容在vscode中运行
            if !mod_jre_jar_path.exists() {
                mod_jre_jar_path = path.join("bin/rt-mod.jar");
            }
            
            if !rt_jar_path.exists() {
                rt_jar_path = path.join("bin/rt.jar");
            }
            
            // 先从rt-mod.jar中查找
            if mod_jre_jar_path.exists() {
                let result = read_class_from_jar(&mod_jre_jar_path.to_string_lossy(), &name_with_ext);
                match result {
                    Some(code) => return Ok(code),
                    None => {
                        // 从rt.jar中查找
                        if rt_jar_path.exists() {
                            let result = read_class_from_jar(&rt_jar_path.to_string_lossy(), &name_with_ext);
                            match result {
                                Some(code) => return Ok(code),
                                None => {
                                    println!("从rt.jar中未找到{}", name_with_ext);
                                    return Err(Throwable::Error(JvmError::NoClassDefFound {
                                        class_name: (name.clone()),
                                        cause: (Some(format!("Class {} not found", name))),
                                        message: (format!("Class {} not found", name)),
                                    }));
                                }
                            }
                        } else {
                            return Err(Throwable::Error(JvmError::NoClassDefFound {
                                class_name: (name.clone()),
                                cause: (Some(format!("rt.jar not found"))),
                                message: (format!("Class {} not found", name)),
                            }));
                        }
                    }
                }
            } else {
                return Err(Throwable::Error(JvmError::NoClassDefFound {
                    class_name: (name.clone()),
                    cause: (Some(format!("rt-mod.jar not found"))),
                    message: (format!("Class {} not found", name)),
                }));
            }
        }
        Err(_e) => {
            return Err(Throwable::Error(JvmError::NoClassDefFound {
                class_name: (name.clone()),
                cause: (Some(format!("Class {} not found", name))),
                message: (format!("Class {} not found", name)),
            }))
        }
    }
}

/***
 * 类加载
 */
pub fn load_class(
    name: &String,
    vm_stack: &mut Vec<StackFrame>,
    heap: &mut Heap,
    metaspace: &mut Metaspace,
) -> Result<Class, Throwable> {
    let buffer: Vec<u8> = get_class_from_disk(name)?;
    let mut cursor = io::Cursor::new(buffer);
    let mut class: Class = Class::new();
    class.magic = get_magic(&mut cursor);
    class.minor_version = get_minor_version(&mut cursor);
    class.major_version = get_major_version(&mut cursor);
    class.constant_pool_count = get_constant_pool_count(&mut cursor);
    class.constant_pool = read_constant_pool_info(class.constant_pool_count, &mut cursor);
    class.access_flags = get_access_flag(&mut cursor);
    class.this_class = get_this_class(&mut cursor);
    class.super_class = get_super_class(&mut cursor);
    class.interface_count = get_interface_count(&mut cursor);
    class.interfaces = get_interface(class.interface_count, &mut cursor);
    class.fields_count = get_field_count(&mut cursor);
    class.field_info = get_field(&class.constant_pool, class.fields_count, &mut cursor, name);
    class.methods_count = get_method_count(&mut cursor);
    class.method_info = get_method(&class.constant_pool, class.methods_count, &mut cursor);
    class.attributes_count = get_attribute_count(&mut cursor);
    class.attribute_info = get_attribute(&class.constant_pool, class.attributes_count, &mut cursor);
    class.class_name = name.clone();
    if class.super_class != 0x00 {
        let class_constant = &class.constant_pool[class.super_class as usize];
        match class_constant {
            ConstantPoolInfo::Class(index) => {
                let name_constant = &class.constant_pool[*index as usize];
                match name_constant {
                    ConstantPoolInfo::Utf8(class_name) => {
                        let super_class_op = metaspace.class_map.get(class_name);
                        if super_class_op.is_none() {
                            let super_class = find_class(&class_name, vm_stack, heap, metaspace)?;
                            class.super_class_id = super_class.id;
                            class.super_class_name = super_class.class_name.clone();
                            for (key, value) in &super_class.field_info {
                                let key_ref: &String = key;
                                /*
                                 * 过滤掉父类中的静态变量和私有变量
                                 */
                                if !class.field_info.contains_key(key_ref)
                                    && value.access_flag & 0x0008 != 0
                                    && value.access_flag & 0x0002 != 0
                                {
                                    class.field_info.insert(key.clone(), value.clone());
                                }
                            }
                        } else {
                            let super_class = &metaspace.classes[*super_class_op.unwrap()];
                            class.super_class_id = super_class.id;
                            class.super_class_name = super_class.class_name.clone();
                        }
                    }
                    _ => panic!("wrong class data"),
                }
            }
            _ => panic!("wrong class data"),
        }
        // load_class(name)
    }
    //do_after_load(&mut class);
    //init_class_id(&mut class);
    //init(name, "<clinit>".to_string(), heap, metaspace);
    Ok(class)
}

/**
 * 类加载完成之后执行初始化静态方法
 */
pub fn init(class_name: &String, method_name: String, heap: &mut Heap, metaspace: &mut Metaspace) {
    let class_id = *metaspace.class_map.get(class_name).unwrap();
    let class = &metaspace.classes[class_id];
    //创建VM
    //找到main方法
    for i in 0..*&class.method_info.len() {
        let method_info = &class.method_info[i];
        //let methond_index = (method_info.name_index as usize) - 1;
        let u8_vec = &class.constant_pool[method_info.name_index as usize];
        match u8_vec {
            ConstantPoolInfo::Utf8(name) => {
                if name == &method_name {
                    let mut stack_frame = create_stack_frame(method_info).unwrap();
                    let mut vm_stack = Vec::new();
                    stack_frame.vm_stack_id = 0;
                    vm_stack.push(stack_frame);
                    // 转换为可变引用（需要 unsafe）
                    do_opcode(&mut vm_stack, heap, metaspace);
                    //execute(vm_stack_id,&mut vm);
                    break;
                }
            }
            _ => panic!("wrong class data"),
        }
    }
}

pub fn parse_method_field(class: &mut Class, method_info_map: &mut HashMap<String, MethodInfo>) {
    let this_class = &class.constant_pool[class.this_class as usize];
    // 设置 class_name
    // match this_class {
    //     ConstantPoolInfo::Class(name_index) => {
    //         //info!("name_index:{:?}", name_index);
    //         let class_name =&class.constant_pool[*name_index as usize];
    //         //info!("class_name:{:?}", class_name);
    //         match class_name {
    //             ConstantPoolInfo::Utf8(name) => {
    //                 class.class_name = name.clone();
    //             }
    //             _ => panic!("wrong constant data type"),
    //         }
    //     }
    //     _ => panic!("wrong constant data type"),
    // }
    //补充方法方法参数解析后信息
    for i in 0..class.methods_count {
        let method_info = &mut class.method_info[i as usize];
        method_info.class_name = (class.class_name).clone();
        let descriptor = &class.constant_pool[method_info.descriptor_index as usize];
        match descriptor {
            ConstantPoolInfo::Utf8(str) => {
                let method_name = &class.constant_pool[method_info.name_index as usize];
                match method_name {
                    ConstantPoolInfo::Utf8(name) => {
                        method_info.method_name = name.clone();
                    }
                    _ => panic!("wrong constant data type"),
                }
                method_info.descriptor = str.clone();
                //info!("method_info.descripto:{:?}", &method_info.descriptor);
                let result = parse_descriptor(&(method_info.descriptor.clone()).into_bytes());
                match result {
                    Ok(Some(parameters)) => {
                        for param in parameters {
                            method_info.param.push(param);
                        }
                    }
                    Ok(None) => {
                        //println!("No parameters");
                    }
                    Err(error) => {
                        println!("Error: {}", error);
                    }
                }
                // add_method(method_info.clone());
                let key = format!(
                    "{}{}{}{}{}",
                    method_info.class_name,
                    ".",
                    method_info.method_name,
                    ".",
                    method_info.descriptor
                );
                method_info.class_id = class.id as u32;
                method_info_map.insert(key, method_info.clone());
            }
            _ => panic!("wrong constant data type"),
        }
    }

    let mut field_index = 0;
    let mut field_offset: u32 = 0;

    //补充方法方法参数解析后信息
    for (_key, field_info) in class.field_info.iter_mut() {
        field_info.field_index = field_index;
        field_info.offset = field_offset;
        field_index += 1;
        match &field_info.data_type {
            DataType::Array {
                element_type: _,
                depth: _,
            } => {
                field_info.value = StackFrameValue::Null;
                field_offset += 4;
            }
            DataType::Byte => {
                field_info.value = StackFrameValue::Byte(0);
                field_offset += 1;
            }
            DataType::Char => {
                field_info.value = StackFrameValue::Char(0);
                field_offset += 2;
            }
            DataType::Double => {
                field_info.value = StackFrameValue::Double(0.0);
                field_offset += 8;
            }
            DataType::Float => {
                field_info.value = StackFrameValue::Float(0.0);
                field_offset += 4;
            }
            DataType::Int => {
                field_info.value = StackFrameValue::Int(0);
                field_offset += 4;
            }
            DataType::Long => {
                field_info.value = StackFrameValue::Long(0);
                field_offset += 8;
            }
            DataType::Reference(_s) => {
                field_info.value = StackFrameValue::Null;
                field_offset += 4;
            }
            DataType::Short => {
                field_info.value = StackFrameValue::Short(0);
                field_offset += 2;
            }
            DataType::Boolean => {
                field_info.value = StackFrameValue::Boolean(false);
                field_offset += 1;
            }
            DataType::Unknown => panic!(),
        }
    }
}

/**
 * 获取魔数
 */
pub fn get_magic(cursor: &mut Cursor<Vec<u8>>) -> u32 {
    cursor.read_u32::<BigEndian>().unwrap()
}

/**
 * 读取一个U32
 */
pub fn read_u32(file: &mut File) -> u32 {
    let mut buffer = [0u8; 4];
    let _ = file.read(&mut buffer);
    u8s_to_u32(&buffer)
}

/**
 * 获取副版本号
 */
pub fn get_minor_version(cursor: &mut Cursor<Vec<u8>>) -> u16 {
    cursor.read_u16::<BigEndian>().unwrap()
}

/**
 * 获取主版本号
 */
pub fn get_major_version(cursor: &mut Cursor<Vec<u8>>) -> u16 {
    cursor.read_u16::<BigEndian>().unwrap()
}

/**
 * 获取常量池大小
 */

pub fn get_constant_pool_count(cursor: &mut Cursor<Vec<u8>>) -> u16 {
    cursor.read_u16::<BigEndian>().unwrap()
}

pub fn get_constant_pool(cnt: u16, file: &mut File) -> Vec<Vec<u8>> {
    let mut ans: Vec<Vec<u8>> = Vec::new();
    let mut i = 1;
    //总数量是cnt - 1;
    while i < cnt {
        let mut buffer = [0u8; 1];
        let _ = file.read(&mut buffer);
        let tag: u8 = buffer[0];
        let mut v: Vec<u8> = Vec::new();
        v.push(tag);
        if tag == 7 {
            let mut buffer = vec![0; 2];
            let _ = file.read(&mut buffer);
            for j in 0..buffer.len() {
                v.push(buffer[j]);
            }
            ans.push(v);
        } else if tag == 9 {
            let mut buffer = [0u8; 4];
            let _ = file.read(&mut buffer);
            for j in 0..buffer.len() {
                v.push(buffer[j]);
            }
            ans.push(v);
        } else if tag == 10 {
            let mut buffer = [0u8; 4];
            let _ = file.read(&mut buffer);
            for j in 0..buffer.len() {
                v.push(buffer[j]);
            }
            ans.push(v);
        } else if tag == 11 {
            let mut buffer = [0u8; 4];
            let _ = file.read(&mut buffer);
            for j in 0..buffer.len() {
                v.push(buffer[j]);
            }
            ans.push(v);
        } else if tag == 8 {
            let mut buffer = [0u8; 2];
            let _ = file.read(&mut buffer);
            for j in 0..buffer.len() {
                v.push(buffer[j]);
            }
            ans.push(v);
        } else if tag == 3 {
            let mut buffer = [0u8; 4];
            let _ = file.read(&mut buffer);
            for j in 0..buffer.len() {
                v.push(buffer[j]);
            }
            ans.push(v);
        } else if tag == 4 {
            let mut buffer = [0u8; 4];
            let _ = file.read(&mut buffer);
            for j in 0..buffer.len() {
                v.push(buffer[j]);
            }
            ans.push(v);
        } else if tag == 5 {
            let mut buffer = [0u8; 8];
            let _ = file.read(&mut buffer).unwrap();
            for j in 0..buffer.len() {
                v.push(buffer[j]);
            }
            ans.push(v);
            i += 1;
        } else if tag == 6 {
            let mut buffer = [0u8; 8];
            let _ = file.read(&mut buffer);
            for j in 0..buffer.len() {
                v.push(buffer[j]);
            }
            ans.push(v);
            i += 1;
        } else if tag == 12 {
            let mut buffer = [0u8; 4];
            let _ = file.read(&mut buffer);
            for j in 0..buffer.len() {
                v.push(buffer[j]);
            }
            ans.push(v);
        } else if tag == 1 {
            let mut buffer = [0u8; 2];
            let _ = file.read(&mut buffer);
            v.push(buffer[0]);
            v.push(buffer[1]);
            let len = u8s_to_u16(&buffer);
            for _j in 0..len {
                let mut buffer = [0u8; 1];
                let _ = file.read(&mut buffer);
                v.push(buffer[0]);
            }
            ans.push(v);
        } else if tag == 15 {
            let mut buffer = [0u8; 3];
            let _ = file.read(&mut buffer);
            for j in 0..buffer.len() {
                v.push(buffer[j]);
            }
            ans.push(v);
        } else if tag == 16 {
            let mut buffer = [0u8; 2];
            let _ = file.read(&mut buffer);
            for j in 0..buffer.len() {
                v.push(buffer[j]);
            }
            ans.push(v);
        } else if tag == 18 {
            let mut buffer = [0u8; 4];
            let _ = file.read(&mut buffer);
            for j in 0..buffer.len() {
                v.push(buffer[j]);
            }
            ans.push(v);
        }
        i += 1;
    }
    ans
}

/**
 * 读取一个U16
 */
pub fn read_u16(mut file: &File) -> u16 {
    let mut buffer = [0u8; 2];
    let _ = file.read(&mut buffer);
    u8s_to_u16(&buffer)
}

/**
 * 读取 access_flag
 */
pub fn get_access_flag(cursor: &mut Cursor<Vec<u8>>) -> u16 {
    cursor.read_u16::<BigEndian>().unwrap()
}

/**
 * 读取 this_class
 */
pub fn get_this_class(cursor: &mut Cursor<Vec<u8>>) -> u16 {
    cursor.read_u16::<BigEndian>().unwrap()
}

/**
 * 读取 super_class
 */
pub fn get_super_class(cursor: &mut Cursor<Vec<u8>>) -> u16 {
    cursor.read_u16::<BigEndian>().unwrap()
}

/**
 * 读取 interface_count
 */
pub fn get_interface_count(cursor: &mut Cursor<Vec<u8>>) -> u16 {
    cursor.read_u16::<BigEndian>().unwrap()
}

/**
 * 读取 interface
 */
pub fn get_interface(cnt: u16, cursor: &mut Cursor<Vec<u8>>) -> Vec<u16> {
    let mut v: Vec<u16> = Vec::new();
    for _j in 0..cnt {
        let interface: u16 = cursor.read_u16::<BigEndian>().unwrap();
        v.push(interface);
    }
    v
}

pub fn get_field_count(cursor: &mut Cursor<Vec<u8>>) -> u16 {
    cursor.read_u16::<BigEndian>().unwrap()
}

pub fn get_field(
    constant_pool: &Vec<ConstantPoolInfo>,
    cnt: u16,
    cursor: &mut Cursor<Vec<u8>>,
    class_name: &String,
) -> LinkedHashMap<String, FieldInfo> {
    let mut v: LinkedHashMap<String, FieldInfo> = LinkedHashMap::new();
    for _j in 0..cnt {
        let mut f: FieldInfo = FieldInfo {
            access_flag: cursor.read_u16::<BigEndian>().unwrap(),
            name_index: cursor.read_u16::<BigEndian>().unwrap(),
            descriptor_index: cursor.read_u16::<BigEndian>().unwrap(),
            attribute_count: cursor.read_u16::<BigEndian>().unwrap(),
            atrributes: Vec::new(),
            value: StackFrameValue::Null,
            field_name: String::from(""),
            data_type: { DataType::Unknown },
            descriptor: String::from(""),
            field_index: 0,
            offset: 0,
            class_name: class_name.clone(),
        };
        let name_utf8 = &constant_pool[f.name_index as usize];
        let field_name = match name_utf8 {
            ConstantPoolInfo::Utf8(name) => name.clone(),
            _ => panic!(),
        };
        f.field_name = field_name;

        let descriptor = &constant_pool[f.descriptor_index as usize];
        //info!("{}===={:?}=====",class.class_name,descriptor);
        match descriptor {
            ConstantPoolInfo::Utf8(str) => {
                f.descriptor = str.clone();
                f.data_type = parse_field_descriptor(f.descriptor.clone().into_bytes());
            }
            _ => panic!(""),
        }

        f.atrributes = get_attribute(constant_pool, f.attribute_count, cursor);
        v.insert(f.field_name.clone(), f);
    }
    v
}

pub fn get_method_count(cursor: &mut Cursor<Vec<u8>>) -> u16 {
    cursor.read_u16::<BigEndian>().unwrap()
}

pub fn get_method(
    constant_pool: &Vec<ConstantPoolInfo>,
    cnt: u16,
    cursor: &mut Cursor<Vec<u8>>,
) -> Vec<MethodInfo> {
    let mut v: Vec<MethodInfo> = Vec::new();
    for _j in 0..cnt {
        let mut m: MethodInfo = MethodInfo {
            access_flag: cursor.read_u16::<BigEndian>().unwrap(),
            name_index: cursor.read_u16::<BigEndian>().unwrap(),
            descriptor_index: cursor.read_u16::<BigEndian>().unwrap(),
            attributes_count: cursor.read_u16::<BigEndian>().unwrap(),
            attributes: Vec::new(),
            param: Vec::new(),
            class_name: String::new(),
            method_name: String::new(),
            descriptor: String::new(),
            class_id: 0,
        };
        m.attributes = get_attribute(constant_pool, m.attributes_count, cursor);
        v.push(m);
    }
    v
}

pub fn get_attribute_count(cursor: &mut Cursor<Vec<u8>>) -> u16 {
    cursor.read_u16::<BigEndian>().unwrap()
}

pub fn get_attribute(
    constant_pool: &Vec<ConstantPoolInfo>,
    cnt: u16,
    cursor: &mut Cursor<Vec<u8>>,
) -> Vec<AttributeInfo> {
    let mut ans: Vec<AttributeInfo> = Vec::new();
    for _i in 0..cnt {
        let attribute_name_index = cursor.read_u16::<BigEndian>().unwrap();
        let attribute_length = cursor.read_u32::<BigEndian>().unwrap();
        let mut attr_info: Vec<u8> = Vec::new();
        for _j in 0..attribute_length {
            attr_info.push(cursor.read_u8().unwrap());
        }
        let mut index: usize = 0;
        let cons = &constant_pool[attribute_name_index as usize];
        match cons {
            ConstantPoolInfo::Utf8(attr_name) => {
                if "Code" == attr_name {
                    let max_stack = u8s_to_u16(&attr_info[0..2]);
                    let max_locals: u16 = u8s_to_u16(&attr_info[2..4]);
                    let code_length: u32 = u8s_to_u32(&attr_info[4..8]);
                    index += 8;
                    let mut code: Vec<u8> = Vec::new();
                    for i in index..index + code_length as usize {
                        code.push(attr_info[i]);
                    }
                    index += code_length as usize;
                    let exception_table_length = u8s_to_u16(&attr_info[index..index + 2]);
                    let mut exception_table = Vec::new();
                    index += 2;
                    for _i in 0..exception_table_length {
                        let start_pc = u8s_to_u16(&attr_info[index..index + 2]);
                        index += 2;
                        let end_pc = u8s_to_u16(&attr_info[index..index + 2]);
                        index += 2;
                        let handler_pc = u8s_to_u16(&attr_info[index..index + 2]);
                        index += 2;
                        let catch_type = u8s_to_u16(&attr_info[index..index + 2]);
                        exception_table.push(ExceptionTable::new(
                            start_pc, end_pc, handler_pc, catch_type,
                        ));
                    }
                    ans.push(AttributeInfo::Code(CodeAttribute::new(
                        max_stack,
                        max_locals,
                        code_length,
                        code,
                        exception_table_length,
                        exception_table,
                    )));
                }
            }
            _ => panic!(),
        }
    }
    ans
}

fn read_constant_pool_info<R: Read>(
    constant_pool_count: u16,
    reader: &mut R,
) -> Vec<ConstantPoolInfo> {
    let mut constant_pool = Vec::new();
    constant_pool.push(ConstantPoolInfo::Utf8(String::from("")));
    let mut index = 1;
    while index <= constant_pool_count - 1 {
        let tag = reader.read_u8().expect("Failed to read constant tag");
        match tag {
            1 => {
                let length = reader
                    .read_u16::<BigEndian>()
                    .expect("Failed to read UTF-8 length");
                let mut data = vec![0; length as usize];
                reader
                    .read_exact(&mut data)
                    .expect("Failed to read UTF-8 data");
                // let result = String::from_utf8_lossy(&data);
                // match result {
                //     Err(e) => {
                //         warn!("Invalid UTF-8 string:{}", e);
                //         constant_pool.push(ConstantPoolInfo::Utf8(String::from("..")));
                //     },
                // };

                let s = u8c::decode_java_mutf8(&data);
                constant_pool.push(ConstantPoolInfo::Utf8(s));
                index += 1;
            }
            3 => {
                constant_pool.push(ConstantPoolInfo::Integer(
                    reader
                        .read_i32::<BigEndian>()
                        .expect("Failed to read integer"),
                ));
                index += 1;
            }
            4 => {
                constant_pool.push(ConstantPoolInfo::Float(
                    reader
                        .read_f32::<BigEndian>()
                        .expect("Failed to read float"),
                ));
                index += 1;
            }
            5 => {
                constant_pool.push(ConstantPoolInfo::Long(
                    reader.read_i64::<BigEndian>().expect("Failed to read long"),
                ));
                constant_pool.push(ConstantPoolInfo::Integer(0));
                index += 2;
            }
            6 => {
                constant_pool.push(ConstantPoolInfo::Double(
                    reader
                        .read_f64::<BigEndian>()
                        .expect("Failed to read double"),
                ));
                constant_pool.push(ConstantPoolInfo::Integer(0));
                index += 2;
            }
            7 => {
                constant_pool.push(ConstantPoolInfo::Class(
                    reader
                        .read_u16::<BigEndian>()
                        .expect("Failed to read class index"),
                ));
                index += 1;
            }
            8 => {
                constant_pool.push(ConstantPoolInfo::String(
                    reader
                        .read_u16::<BigEndian>()
                        .expect("Failed to read string index"),
                ));
                index += 1;
            }
            9 => {
                let class_index = reader
                    .read_u16::<BigEndian>()
                    .expect("Failed to read fieldref class index");
                let name_and_type_index = reader
                    .read_u16::<BigEndian>()
                    .expect("Failed to read fieldref name and type index");
                constant_pool.push(ConstantPoolInfo::Fieldref(class_index, name_and_type_index));
                index += 1;
            }
            10 => {
                let class_index = reader
                    .read_u16::<BigEndian>()
                    .expect("Failed to read methodref class index");
                let name_and_type_index = reader
                    .read_u16::<BigEndian>()
                    .expect("Failed to read methodref name and type index");
                constant_pool.push(ConstantPoolInfo::Methodref(
                    class_index,
                    name_and_type_index,
                ));
                index += 1;
            }
            11 => {
                let class_index = reader
                    .read_u16::<BigEndian>()
                    .expect("Failed to read interface methodref class index");
                let name_and_type_index = reader
                    .read_u16::<BigEndian>()
                    .expect("Failed to read interface methodref name and type index");
                constant_pool.push(ConstantPoolInfo::InterfaceMethodref(
                    class_index,
                    name_and_type_index,
                ));
                index += 1;
            }
            12 => {
                let name_index = reader
                    .read_u16::<BigEndian>()
                    .expect("Failed to read name and type name index");
                let descriptor_index = reader
                    .read_u16::<BigEndian>()
                    .expect("Failed to read name and type descriptor index");
                constant_pool.push(ConstantPoolInfo::NameAndType(name_index, descriptor_index));
                index += 1;
            }
            15 => {
                let reference_kind = reader
                    .read_u8()
                    .expect("Failed to read method handle reference kind");
                let reference_index = reader
                    .read_u16::<BigEndian>()
                    .expect("Failed to read method handle reference index");
                constant_pool.push(ConstantPoolInfo::MethodHandle(
                    reference_kind,
                    reference_index,
                ));
                index += 1;
            }
            16 => {
                constant_pool.push(ConstantPoolInfo::MethodType(
                    reader
                        .read_u16::<BigEndian>()
                        .expect("Failed to read method type descriptor index"),
                ));
                index += 1;
            }
            18 => {
                let bootstrap_method_attr_index = reader
                    .read_u16::<BigEndian>()
                    .expect("Failed to read invoke dynamic bootstrap method attribute index");
                let name_and_type_index = reader
                    .read_u16::<BigEndian>()
                    .expect("Failed to read invoke dynamic name and type index");
                constant_pool.push(ConstantPoolInfo::InvokeDynamic(
                    bootstrap_method_attr_index,
                    name_and_type_index,
                ));
                index += 1;
            }
            19 => {
                constant_pool.push(ConstantPoolInfo::Module(
                    reader
                        .read_u16::<BigEndian>()
                        .expect("Failed to read module index"),
                ));
                index += 1;
            }
            20 => {
                constant_pool.push(ConstantPoolInfo::Package(
                    reader
                        .read_u16::<BigEndian>()
                        .expect("Failed to read package index"),
                ));
                index += 1;
            }
            21 => {
                let method_handle_reference_kind = reader
                    .read_u8()
                    .expect("Failed to read method handle reference kind");
                let method_handle_reference_index = reader
                    .read_u16::<BigEndian>()
                    .expect("Failed to read method handle reference index");
                constant_pool.push(ConstantPoolInfo::MethodPointer(
                    method_handle_reference_kind,
                    method_handle_reference_index,
                ));
                index += 1;
            }
            22 => {
                let invoke_static_dynamic_name_and_type_index = reader
                    .read_u16::<BigEndian>()
                    .expect("Failed to read invoke static dynamic name and type index");
                let invoke_static_dynamic_bootstrap_method_attr_index =
                    reader.read_u16::<BigEndian>().expect(
                        "Failed to read invoke static dynamic bootstrap method attribute index",
                    );
                constant_pool.push(ConstantPoolInfo::InvokeStaticDynamic(
                    invoke_static_dynamic_bootstrap_method_attr_index,
                    invoke_static_dynamic_name_and_type_index,
                ));
                index += 1;
            }
            23 => {
                let bootstrap_method_attr_index = reader
                    .read_u16::<BigEndian>()
                    .expect("Failed to read bootstrap method attribute index");
                let name_and_type_index = reader
                    .read_u16::<BigEndian>()
                    .expect("Failed to read bootstrap method name and type index");
                constant_pool.push(ConstantPoolInfo::BootstrapMethod(
                    bootstrap_method_attr_index,
                    name_and_type_index,
                ));
                index += 1;
            }
            24 => {
                let method_type_reference_index = reader
                    .read_u16::<BigEndian>()
                    .expect("Failed to read method type reference index");
                constant_pool.push(ConstantPoolInfo::MethodTypeReference(
                    method_type_reference_index,
                ));
                index += 1;
            }
            // 添加更多常量类型的处理
            _ => panic!("Invalid constant pool tag: {}", tag),
        }
        //index += 1;
        // constant_pool_count -= 1;
    }

    constant_pool
}

pub fn find_class<'a, 'b>(
    class_name: &'a String,
    vm_stack: &'b mut Vec<StackFrame>,
    heap: &'b mut Heap,
    metaspace: &'a mut Metaspace,
) -> Result<&'a mut Class, Throwable> {
    let (class, flag) = {
        let class_op = metaspace.class_map.get(class_name);
        if class_op.is_none() {
            let mut class = load_class(class_name, vm_stack, heap, metaspace)?;
            let id = metaspace.classes.len();
            class.id = id;
            //class.class_name = class_name.clone();
            metaspace.classes.push(class);
            parse_method_field(&mut metaspace.classes[id], &mut metaspace.method_area);
            metaspace.class_map.insert(class_name.clone(), id);
            init(class_name, "<clinit>".to_string(), heap, metaspace);
            (&mut metaspace.classes[id], true)
        } else {
            (&mut metaspace.classes[class_op.unwrap().clone()], false)
        }
    };
    if flag {}
    Ok(class)
}
