use crate::classfile::class::AttributeInfo;
use crate::classfile::class::Class;
use crate::classfile::class::CodeAttribute;
use crate::classfile::class::ConstantPoolInfo;
use crate::classfile::class::ExceptionTable;
use crate::classfile::class::FieldInfo;
use crate::classfile::class::MethodInfo;
use crate::common::param::DataType;
use crate::common::stack_frame::*;
use crate::common::value::StackFrameValue;
use crate::interpreter::instructions::op_code::op_code::execute;
use crate::runtime::runtime_data_area::add_method;
use crate::runtime::runtime_data_area::class_exists;
use crate::runtime::runtime_data_area::init_class_id;
use crate::utils::u8c::u8s_to_u16;
use crate::utils::u8c::u8s_to_u32;
use byteorder::{BigEndian, ReadBytesExt};
use log::info;
use log::warn;
use std::collections::HashMap;
use std::env;
use std::fs::{self, File};
use std::io;
use std::io::Cursor;
use std::io::Read;
use std::path::Path;
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

// fn read_class_from_jar(jar_path: &str, class_name: &str) -> Result<Vec<u8>, String> {
//     let file: File =
//         File::open(jar_path).map_err(|e| format!("Error opening JAR file: {}", e))?;
//     let mut archive: ZipArchive<File> =
//         ZipArchive::new(file).map_err(|e| format!("Error reading ZIP archive: {}", e))?;
//     for i in 0..archive.len() {
//         let mut entry = archive
//             .by_index(i)
//             .map_err(|e| format!("Error reading entry {}: {}", i, e))?;
//         if entry.name().eq(class_name) {
//             let mut buffer = Vec::new();
//             entry
//                 .read_to_end(&mut buffer)
//                 .map_err(|e| format!("Error reading class file: {}", e))?;
//             return Ok(buffer);
//         }
//     }
//     Err(format!("Class '{}' not found in the JAR file", class_name))
// }

// fn get_rt_class(name: &String) -> Option<Vec<u8>> {
//     match env::current_dir() {
//         Ok(path) => {
//             let mut user_class_path = path.as_path().to_str().unwrap().to_string();
//             user_class_path.push_str("/jre/out/");
//             user_class_path.push_str(name);
//             user_class_path.push_str(".class");
//             info!("user class path:{}", user_class_path);
//             let mut file = fs::File::open(user_class_path).unwrap();
//             let mut buffer = Vec::new();
//             let _ = file.read_to_end(&mut buffer);
//             Some(buffer)
//         }
//         Err(_e) =>panic!()
//     }
// }

// fn get_rt_class_jar(name: &String) -> Result<Vec<u8>, String> {
//     match env::var("JAVA_HOME") {
//         Ok(mut home_path) => {
//             let path: String = name.clone() + ".class";
//             info!("{:?}", path);
//             home_path.push_str(&String::from("/jre/lib/rt.jar"));
//             read_class_from_jar(&home_path, &path)
//         }
//         Err(_) => Err(format!("Class '{}' not found in the JAR file", name)),
//     }
// }

fn get_class_from_disk(name: &String) -> Vec<u8> {
    match env::current_dir() {
        Ok(path) => {
            let mut jre_class_path = path.as_path().to_str().unwrap().to_string();
            jre_class_path.push_str("/jre/out/");
            jre_class_path.push_str(name);
            jre_class_path.push_str(".class");
            let class_path = Path::new(&jre_class_path);
            if class_path.exists() {
                let mut file = fs::File::open(jre_class_path).unwrap();
                let mut buffer = Vec::new();
                let _ = file.read_to_end(&mut buffer);
                buffer
            } else {
                let mut user_class_path = path.as_path().to_str().unwrap().to_string();
                user_class_path.push_str("/test/out/");
                user_class_path.push_str(name);
                user_class_path.push_str(".class");
                let class_path = Path::new(&user_class_path);
                if class_path.exists() {
                    let mut file = fs::File::open(user_class_path).unwrap();
                    let mut buffer = Vec::new();
                    let _ = file.read_to_end(&mut buffer);
                    buffer
                } else {
                    panic!("class ：{} not found", name)
                }
            }
        }
        Err(_e) => panic!(),
    }
}

/***
 * 类加载
 */
pub fn load_class(name: &String) -> Class {
    let buffer: Vec<u8> = get_class_from_disk(name);
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
    if class.super_class != 0x00 {
        let class_constant = class.constant_pool.get(&class.super_class).unwrap();
        match class_constant {
            ConstantPoolInfo::Class(index) => {
                let name_constant = class.constant_pool.get(index).unwrap();
                match name_constant {
                    ConstantPoolInfo::Utf8(class_name) => {
                        if !class_exists(class_name) {
                            load_class(class_name);
                        }
                    }
                    _ => panic!("wrong class data"),
                }
            }
            _ => panic!("wrong class data"),
        }
        // load_class(name)
    }
    class.interface_count = get_interface_count(&mut cursor);
    class.interfaces = get_interface(class.interface_count, &mut cursor);
    class.fields_count = get_field_count(&mut cursor);
    class.field_info = get_field(&class.constant_pool, class.fields_count, &mut cursor);
    class.methods_count = get_method_count(&mut cursor);
    class.method_info = get_method(&class.constant_pool, class.methods_count, &mut cursor);
    class.attributes_count = get_attribute_count(&mut cursor);
    class.attribute_info = get_attribute(&class.constant_pool, class.attributes_count, &mut cursor);
    do_after_load(&mut class);
    init_class_id(&mut class);
    init(&mut class, "<clinit>".to_string());
    class
}

/**
 * 类加载完成之后执行初始化静态方法
 */
pub fn init(class: &mut Class, method_name: String) {
    //let class= get_or_load_class(&clazz.class_name);
    //创建VM
    //找到main方法
    for i in 0..*&class.method_info.len() {
        let method_info = &class.method_info[i];
        //let methond_index = (method_info.name_index as usize) - 1;
        let u8_vec = class.constant_pool.get(&method_info.name_index).unwrap();
        match u8_vec {
            ConstantPoolInfo::Utf8(name) => {
                //println!("method:{}", &name);
                //info!("{}", name);
                //创建虚拟机栈，并创建第一个栈帧
                if name == &method_name {
                    let stack_frame = create_stack_frame_with_class(method_info, class).unwrap();
                    // info!("{:?}",stack_frame);
                    //let vm_stack_id = (&stack_frame).vm_stack_id;
                    //    let stack_frame_clone = stack_frame.clone();
                    let vm_stack_id = push_stack_frame(stack_frame);
                    execute(vm_stack_id);
                }
            }
            _ => panic!("wrong class data"),
        }
    }
}

fn do_after_load(class: &mut Class) {
    let this_class = class.constant_pool.get(&class.this_class).unwrap();

    //info!("this_class:{:?}", this_class);
    // 设置 class_name
    match this_class {
        ConstantPoolInfo::Class(name_index) => {
            //info!("name_index:{:?}", name_index);
            let class_name = class.constant_pool.get(name_index).unwrap();
            //info!("class_name:{:?}", class_name);
            match class_name {
                ConstantPoolInfo::Utf8(name) => {
                    class.class_name = name.clone();
                }
                _ => panic!("wrong constant data type"),
            }
        }
        _ => panic!("wrong constant data type"),
    }

    // java.lang.Object 没有父类
    if class.super_class != 0 {
        let super_class = class.constant_pool.get(&class.super_class).unwrap();
        match super_class {
            ConstantPoolInfo::Class(name_index) => {
                let class_name = class.constant_pool.get(name_index).unwrap();
                match class_name {
                    ConstantPoolInfo::Utf8(name) => {
                        class.super_class_name = name.clone();
                    }
                    _ => panic!("wrong constant data type"),
                }
            }
            _ => panic!("wrong constant data type"),
        }
    }
    //补充方法方法参数解析后信息
    for i in 0..class.methods_count {
        let method_info = class.method_info.get_mut(i as usize).unwrap();
        method_info.class_name = (class.class_name).clone();
        let descriptor = class
            .constant_pool
            .get(&method_info.descriptor_index)
            .unwrap();
        match descriptor {
            ConstantPoolInfo::Utf8(str) => {
                let method_name = class.constant_pool.get(&method_info.name_index).unwrap();
                match method_name {
                    ConstantPoolInfo::Utf8(name) => {
                        method_info.method_name = name.clone();
                    }
                    _ => panic!("wrong constant data type"),
                }
                method_info.descriptor = str.clone();
                //info!("method_info.descripto:{:?}", &method_info.descriptor);
                let result = parse_descriptor(&(str.clone().into_bytes()));
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
                add_method(method_info.clone());
            }
            _ => panic!("wrong constant data type"),
        }
    }

    //补充方法方法参数解析后信息
    for (_key, field_info) in class.field_info.iter_mut() {
        let descriptor = class
            .constant_pool
            .get(&field_info.descriptor_index)
            .unwrap();
        match descriptor {
            ConstantPoolInfo::Utf8(str) => {
                field_info.descriptor = str.clone();
                let result: Result<Option<Vec<DataType>>, String> =
                    parse_descriptor(&(str.clone().into_bytes()));
                match result {
                    Ok(Some(parameters)) => {
                        if let Some(param) = parameters.into_iter().next() {
                            field_info.data_type = param.clone();
                            match param {
                                DataType::Array {
                                    element_type: _,
                                    depth: _,
                                } => field_info.value = StackFrameValue::Null,
                                DataType::Byte => {
                                    field_info.value = StackFrameValue::Byte(0);
                                }
                                DataType::Char => {
                                    field_info.value = StackFrameValue::Char(0);
                                }
                                DataType::Double => {
                                    field_info.value = StackFrameValue::Double(0.0);
                                }
                                DataType::Float => {
                                    field_info.value = StackFrameValue::Float(0.0);
                                }
                                DataType::Int => {
                                    field_info.value = StackFrameValue::Int(0);
                                }
                                DataType::Long => {
                                    field_info.value = StackFrameValue::Long(0);
                                }
                                DataType::Reference(_s) => {
                                    field_info.value = StackFrameValue::Null;
                                }
                                DataType::Short => {
                                    field_info.value = StackFrameValue::Short(0);
                                }
                                DataType::Boolean => {
                                    field_info.value = StackFrameValue::Boolean(false);
                                }
                                DataType::Unknown => panic!(),
                            }
                        }
                    }
                    Ok(None) => {
                        //println!("No parameters");
                    }
                    Err(error) => {
                        println!("Error: {}", error);
                    }
                }
            }
            _ => panic!("wrong constant data type"),
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
    constant_pool: &HashMap<u16, ConstantPoolInfo>,
    cnt: u16,
    cursor: &mut Cursor<Vec<u8>>,
) -> HashMap<String, FieldInfo> {
    let mut v: HashMap<String, FieldInfo> = HashMap::new();
    for _j in 0..cnt {
        let mut f: FieldInfo = FieldInfo {
            access_flag: cursor.read_u16::<BigEndian>().unwrap(),
            name_index: cursor.read_u16::<BigEndian>().unwrap(),
            descriptor_index: cursor.read_u16::<BigEndian>().unwrap(),
            attribute_count: cursor.read_u16::<BigEndian>().unwrap(),
            atrributes: Vec::new(),
            value: StackFrameValue::Null,
            field_name: String::from(""),
            data_type: DataType::Unknown,
            descriptor: String::from(""),
        };

        let name_utf8 = constant_pool.get(&f.name_index).unwrap();
        let field_name = match name_utf8 {
            ConstantPoolInfo::Utf8(name) => name.clone(),
            _ => panic!(),
        };
        f.field_name = field_name;
        f.atrributes = get_attribute(constant_pool, f.attribute_count, cursor);
        v.insert(f.field_name.clone(), f);
    }
    v
}

pub fn get_method_count(cursor: &mut Cursor<Vec<u8>>) -> u16 {
    cursor.read_u16::<BigEndian>().unwrap()
}

pub fn get_method(
    constant_pool: &HashMap<u16, ConstantPoolInfo>,
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
    constant_pool: &HashMap<u16, ConstantPoolInfo>,
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
        let cons = constant_pool.get(&attribute_name_index).unwrap();
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
    mut constant_pool_count: u16,
    reader: &mut R,
) -> HashMap<u16, ConstantPoolInfo> {
    let mut constant_pool = HashMap::new();

    let mut index: u16 = 1;
    while constant_pool_count - 1 > 0 {
        let tag = reader.read_u8().expect("Failed to read constant tag");
        // info!("constant_pool_tag:{:?}", tag);
        match tag {
            1 => {
                let length = reader
                    .read_u16::<BigEndian>()
                    .expect("Failed to read UTF-8 length");
                let mut data = vec![0; length as usize];
                reader
                    .read_exact(&mut data)
                    .expect("Failed to read UTF-8 data");
                let utf8_string = String::from_utf8(data).expect("Invalid UTF-8 string");
                constant_pool.insert(index, ConstantPoolInfo::Utf8(utf8_string));
                index += 1;
            }
            3 => {
                constant_pool.insert(
                    index,
                    ConstantPoolInfo::Integer(
                        reader
                            .read_i32::<BigEndian>()
                            .expect("Failed to read integer"),
                    ),
                );
                index += 1;
            }
            4 => {
                constant_pool.insert(
                    index,
                    ConstantPoolInfo::Float(
                        reader
                            .read_f32::<BigEndian>()
                            .expect("Failed to read float"),
                    ),
                );
                index += 1;
            }
            5 => {
                constant_pool.insert(
                    index,
                    ConstantPoolInfo::Long(
                        reader.read_i64::<BigEndian>().expect("Failed to read long"),
                    ),
                );
                constant_pool_count -= 1;
                index += 2;
            }
            6 => {
                constant_pool.insert(
                    index,
                    ConstantPoolInfo::Double(
                        reader
                            .read_f64::<BigEndian>()
                            .expect("Failed to read double"),
                    ),
                );
                constant_pool_count -= 1;
                index += 2;
            }
            7 => {
                constant_pool.insert(
                    index,
                    ConstantPoolInfo::Class(
                        reader
                            .read_u16::<BigEndian>()
                            .expect("Failed to read class index"),
                    ),
                );
                index += 1;
            }
            8 => {
                constant_pool.insert(
                    index,
                    ConstantPoolInfo::String(
                        reader
                            .read_u16::<BigEndian>()
                            .expect("Failed to read string index"),
                    ),
                );
                index += 1;
            }
            9 => {
                let class_index = reader
                    .read_u16::<BigEndian>()
                    .expect("Failed to read fieldref class index");
                let name_and_type_index = reader
                    .read_u16::<BigEndian>()
                    .expect("Failed to read fieldref name and type index");
                constant_pool.insert(
                    index,
                    ConstantPoolInfo::Fieldref(class_index, name_and_type_index),
                );
                index += 1;
            }
            10 => {
                let class_index = reader
                    .read_u16::<BigEndian>()
                    .expect("Failed to read methodref class index");
                let name_and_type_index = reader
                    .read_u16::<BigEndian>()
                    .expect("Failed to read methodref name and type index");
                constant_pool.insert(
                    index,
                    ConstantPoolInfo::Methodref(class_index, name_and_type_index),
                );
                index += 1;
            }
            11 => {
                let class_index = reader
                    .read_u16::<BigEndian>()
                    .expect("Failed to read interface methodref class index");
                let name_and_type_index = reader
                    .read_u16::<BigEndian>()
                    .expect("Failed to read interface methodref name and type index");
                constant_pool.insert(
                    index,
                    ConstantPoolInfo::InterfaceMethodref(class_index, name_and_type_index),
                );
                index += 1;
            }
            12 => {
                let name_index = reader
                    .read_u16::<BigEndian>()
                    .expect("Failed to read name and type name index");
                let descriptor_index = reader
                    .read_u16::<BigEndian>()
                    .expect("Failed to read name and type descriptor index");
                constant_pool.insert(
                    index,
                    ConstantPoolInfo::NameAndType(name_index, descriptor_index),
                );
                index += 1;
            }
            15 => {
                let reference_kind = reader
                    .read_u8()
                    .expect("Failed to read method handle reference kind");
                let reference_index = reader
                    .read_u16::<BigEndian>()
                    .expect("Failed to read method handle reference index");
                constant_pool.insert(
                    index,
                    ConstantPoolInfo::MethodHandle(reference_kind, reference_index),
                );
                index += 1;
            }
            16 => {
                constant_pool.insert(
                    index,
                    ConstantPoolInfo::MethodType(
                        reader
                            .read_u16::<BigEndian>()
                            .expect("Failed to read method type descriptor index"),
                    ),
                );
                index += 1;
            }
            18 => {
                let bootstrap_method_attr_index = reader
                    .read_u16::<BigEndian>()
                    .expect("Failed to read invoke dynamic bootstrap method attribute index");
                let name_and_type_index = reader
                    .read_u16::<BigEndian>()
                    .expect("Failed to read invoke dynamic name and type index");
                constant_pool.insert(
                    index,
                    ConstantPoolInfo::InvokeDynamic(
                        bootstrap_method_attr_index,
                        name_and_type_index,
                    ),
                );
                index += 1;
            }
            19 => {
                constant_pool.insert(
                    index,
                    ConstantPoolInfo::Module(
                        reader
                            .read_u16::<BigEndian>()
                            .expect("Failed to read module index"),
                    ),
                );
                index += 1;
            }
            20 => {
                constant_pool.insert(
                    index,
                    ConstantPoolInfo::Package(
                        reader
                            .read_u16::<BigEndian>()
                            .expect("Failed to read package index"),
                    ),
                );
                index += 1;
            }
            21 => {
                let method_handle_reference_kind = reader
                    .read_u8()
                    .expect("Failed to read method handle reference kind");
                let method_handle_reference_index = reader
                    .read_u16::<BigEndian>()
                    .expect("Failed to read method handle reference index");
                constant_pool.insert(
                    index,
                    ConstantPoolInfo::MethodPointer(
                        method_handle_reference_kind,
                        method_handle_reference_index,
                    ),
                );
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
                constant_pool.insert(
                    index,
                    ConstantPoolInfo::InvokeStaticDynamic(
                        invoke_static_dynamic_bootstrap_method_attr_index,
                        invoke_static_dynamic_name_and_type_index,
                    ),
                );
                index += 1;
            }
            23 => {
                let bootstrap_method_attr_index = reader
                    .read_u16::<BigEndian>()
                    .expect("Failed to read bootstrap method attribute index");
                let name_and_type_index = reader
                    .read_u16::<BigEndian>()
                    .expect("Failed to read bootstrap method name and type index");
                constant_pool.insert(
                    index,
                    ConstantPoolInfo::BootstrapMethod(
                        bootstrap_method_attr_index,
                        name_and_type_index,
                    ),
                );
                index += 1;
            }
            24 => {
                let method_type_reference_index = reader
                    .read_u16::<BigEndian>()
                    .expect("Failed to read method type reference index");
                constant_pool.insert(
                    index,
                    ConstantPoolInfo::MethodTypeReference(method_type_reference_index),
                );
                index += 1;
            }
            // 添加更多常量类型的处理
            _ => panic!("Invalid constant pool tag: {}", tag),
        }
        //index += 1;
        constant_pool_count -= 1;
    }

    constant_pool
}
