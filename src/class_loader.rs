pub mod class_loader {
    //文件名（需要在main中先声明）+Mod名+引入对象()
    use crate::class::class::MethodParameter;
    use crate::class::class::Class;
    use crate::class::class::CodeAttribute;
    use std::fs::{self, File};
    use std::io::{prelude::*, Bytes};
    use std::thread::panicking;
    use crate::u8c::u8c::u8s_to_u16;
    use crate::u8c::u8c::u8s_to_u32;
    use crate::u8c::u8c::u8s_to_u64;
    use crate::class::class::FieldInfo;
    use crate::class::class::MethodInfo;
    use crate::class::class::AttributeInfo;

    fn parse_method_descriptor(
        descriptor: &Vec<u8>,
    ) -> Result<Option<Vec<MethodParameter>>, String> {
        let mut index = 0;
        let descriptor_length = descriptor.len();
        let mut parameters: Vec<MethodParameter> = Vec::new();
        while index < descriptor_length {
            let descriptor_char = descriptor[index] as char;
            if(descriptor_char == '(' || descriptor_char == ')'){
                index += 1;
                continue;
            }
            match descriptor_char {
                'B' => parameters.push(MethodParameter::Byte),
                'C' => parameters.push(MethodParameter::Char),
                'D' => parameters.push(MethodParameter::Double),
                'F' => parameters.push(MethodParameter::Float),
                'I' => parameters.push(MethodParameter::Int),
                'J' => parameters.push(MethodParameter::Long),
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
                    parameters.push(MethodParameter::Reference(class_name));
                }
                'S' => parameters.push(MethodParameter::Short),
                'Z' => parameters.push(MethodParameter::Boolean),
                '[' => {
                    // Handle array type parameters
                    let mut array_depth = 1;
                    while index + 1 < descriptor_length && descriptor[index + 1] as char == '[' {
                        array_depth += 1;
                        index += 1;
                    }
                    let element_type = match descriptor[index] as char {
                        'B' => MethodParameter::Byte,
                        'C' => MethodParameter::Char,
                        'D' => MethodParameter::Double,
                        'F' => MethodParameter::Float,
                        'I' => MethodParameter::Int,
                        'J' => MethodParameter::Long,
                        'L' => {
                            let mut class_name = String::new();
                            while index < descriptor_length {
                                index += 1;
                                if descriptor[index] as char == ';' {
                                    break;
                                }
                                class_name.push(descriptor[index] as char);
                            }
                            MethodParameter::Reference(class_name)
                        }
                        'S' => MethodParameter::Short,
                        'Z' => MethodParameter::Boolean,
                        _ => return Err("Unknown array element type".to_string()),
                    };
                    parameters.push(MethodParameter::Array {
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

    /***
     * 类加载
     */
    pub fn load_class(name: &String) -> Class {
        //设置class_path
        let class_path = String::from("E:/test/");
        let appendix = String::from(".class");
        let mut path = class_path + &name + &appendix;
        let rt_path = String::from("E:/tomato/rt/") + &name + &appendix;
        match fs::metadata(&path) {
            Ok(metadata) => {
                if !metadata.is_file() {
                    path = rt_path;
                }
            }
            Err(_) => {
                path = rt_path;
            }
        }
        let mut file = fs::File::open(path).unwrap();
        let mut class: Class = Class::new();
        class.magic = get_magic(&mut file);
        class.minor_version = get_minor_version(&mut file);
        class.major_version = get_major_version(&mut file);
        class.constant_pool_count = get_constant_pool_count(&mut file);
        class.constant_pool = get_constant_pool(class.constant_pool_count, &mut file);
        class.access_flags = get_access_flag(&mut file);
        class.this_class = get_this_class(&mut file);
        class.super_class = get_super_class(&mut file);
        class.interface_count = get_interface_count(&mut file);
        class.interfaces = get_interface(class.interface_count, &mut file);
        class.fields_count = get_field_count(&mut file);
        class.field_info = get_field(class.fields_count, &mut file);
        class.methods_count = get_method_count(&mut file);
        class.method_info = get_method(class.methods_count, &mut file);
        class.attributes_count = get_attribute_count(&mut file);
        class.attribute_info = get_attribute(class.attributes_count, &mut file);

        //补充方法方法参数解析后信息
        for i in 0..class.methods_count {
            let method_info = class.method_info.get_mut(i as usize).unwrap();
            //let index = method_info.descriptor_index as usize - 1;
            let descriptor = class
                .constant_pool
                .get(method_info.descriptor_index as usize - 1)
                .unwrap();
            let len = u8s_to_u16(&descriptor[1..3]);
            let bytes = &descriptor[3..(3 + len as usize)];
            let result = parse_method_descriptor(&bytes.to_vec());
            match result {
                Ok(Some(parameters)) => {
                    for param in parameters {
                        method_info.param.push(param);
                    }
                }
                Ok(None) => {
                    println!("No parameters");
                }
                Err(error) => {
                    println!("Error: {}", error);
                }
            }
        }
        return class;
    }

      /**
     * 获取魔数
     */
    pub fn get_magic(file: &mut File) -> u32 {
        let mut buffer = [0u8; 4];
        file.read(&mut buffer).unwrap();
        return u8s_to_u32(&buffer);
    }

    /**
     * 读取一个U32
     */
    pub fn read_u32(file: &mut File) -> u32 {
        let mut buffer = [0u8; 4];
        file.read(&mut buffer).unwrap();
        return u8s_to_u32(&buffer);
    }

    /**
     * 获取副版本号
     */
    pub fn get_minor_version(file: &mut File) -> u16 {
        let mut buffer = [0u8; 2];
        file.read(&mut buffer).unwrap();
        return u8s_to_u16(&buffer);
    }

    /**
     * 获取主版本号
     */
    pub fn get_major_version(file: &mut File) -> u16 {
        let mut buffer = [0u8; 2];
        file.read(&mut buffer).unwrap();
        return u8s_to_u16(&buffer);
    }

    /**
     * 获取常量池大小
     */

    pub fn get_constant_pool_count(file: &mut File) -> u16 {
        let mut buffer = [0u8; 2];
        file.read(&mut buffer).unwrap();
        let a: u16 = buffer[0] as u16;
        let b: u16 = buffer[1] as u16;
        return a * 256 + b;
    }

    pub fn get_constant_pool(cnt: u16, file: &mut File) -> Vec<Vec<u8>> {
        let mut ans: Vec<Vec<u8>> = Vec::new();
        let mut i = 1;
        //总数量是cnt - 1;
        while i < cnt {
            let mut buffer = [0u8; 1];
            file.read(&mut buffer).unwrap();
            let tag: u8 = buffer[0];
            let mut v: Vec<u8> = Vec::new();
            v.push(tag);
            if tag == 7 {
                let mut buffer = vec![0; 2];
                file.read(&mut buffer).unwrap();
                for j in 0..buffer.len() {
                    v.push(buffer[j]);
                }
                ans.push(v);
            } else if tag == 9 {
                let mut buffer = [0u8; 4];
                file.read(&mut buffer).unwrap();
                for j in 0..buffer.len() {
                    v.push(buffer[j]);
                }
                ans.push(v);
            } else if tag == 10 {
                let mut buffer = [0u8; 4];
                file.read(&mut buffer).unwrap();
                for j in 0..buffer.len() {
                    v.push(buffer[j]);
                }
                ans.push(v);
            } else if tag == 11 {
                let mut buffer = [0u8; 4];
                file.read(&mut buffer).unwrap();
                for j in 0..buffer.len() {
                    v.push(buffer[j]);
                }
                ans.push(v);
            } else if tag == 8 {
                let mut buffer = [0u8; 2];
                file.read(&mut buffer).unwrap();
                for j in 0..buffer.len() {
                    v.push(buffer[j]);
                }
                ans.push(v);
            } else if tag == 3 {
                let mut buffer = [0u8; 4];
                file.read(&mut buffer).unwrap();
                for j in 0..buffer.len() {
                    v.push(buffer[j]);
                }
                ans.push(v);
            } else if tag == 4 {
                let mut buffer = [0u8; 4];
                file.read(&mut buffer).unwrap();
                for j in 0..buffer.len() {
                    v.push(buffer[j]);
                }
                ans.push(v);
            } else if tag == 5 {
                let mut buffer = [0u8; 8];
                file.read(&mut buffer).unwrap();
                for j in 0..buffer.len() {
                    v.push(buffer[j]);
                }
                ans.push(v);
                i = i + 1;
            } else if tag == 6 {
                let mut buffer = [0u8; 8];
                file.read(&mut buffer).unwrap();
                for j in 0..buffer.len() {
                    v.push(buffer[j]);
                }
                ans.push(v);
                i = i + 1;
            } else if tag == 12 {
                let mut buffer = [0u8; 4];
                file.read(&mut buffer).unwrap();
                for j in 0..buffer.len() {
                    v.push(buffer[j]);
                }
                ans.push(v);
            } else if tag == 1 {
                let mut buffer = [0u8; 2];
                file.read(&mut buffer).unwrap();
                v.push(buffer[0]);
                v.push(buffer[1]);
                let len = u8s_to_u16(&buffer);
                for j in 0..len {
                    let mut buffer = [0u8; 1];
                    file.read(&mut buffer).unwrap();
                    v.push(buffer[0]);
                }
                ans.push(v);
            } else if tag == 15 {
                let mut buffer = [0u8; 3];
                file.read(&mut buffer).unwrap();
                for j in 0..buffer.len() {
                    v.push(buffer[j]);
                }
                ans.push(v);
            } else if tag == 16 {
                let mut buffer = [0u8; 2];
                file.read(&mut buffer).unwrap();
                for j in 0..buffer.len() {
                    v.push(buffer[j]);
                }
                ans.push(v);
            } else if tag == 18 {
                let mut buffer = [0u8; 4];
                file.read(&mut buffer).unwrap();
                for j in 0..buffer.len() {
                    v.push(buffer[j]);
                }
                ans.push(v);
            }
            i = i + 1;
        }
        return ans;
    }

    /**
     * 读取一个U16
     */
    pub fn read_u16(mut file: &File) -> u16 {
        let mut buffer = [0u8; 2];
        file.read(&mut buffer).unwrap();
        return u8s_to_u16(&buffer);
    }

    /**
     * 读取 access_flag
     */
    pub fn get_access_flag(file: &mut File) -> u16 {
        let mut buffer = [0u8; 2];
        file.read(&mut buffer).unwrap();
        return u8s_to_u16(&buffer);
    }

    /**
     * 读取 this_class
     */
    pub fn get_this_class(file: &mut File) -> u16 {
        let mut buffer = [0u8; 2];
        file.read(&mut buffer).unwrap();
        return u8s_to_u16(&buffer);
    }

    /**
     * 读取 super_class
     */
    pub fn get_super_class(file: &mut File) -> u16 {
        let mut buffer = [0u8; 2];
        file.read(&mut buffer).unwrap();
        return u8s_to_u16(&buffer);
    }

    /**
     * 读取 interface_count
     */
    pub fn get_interface_count(file: &mut File) -> u16 {
        let mut buffer = [0u8; 2];
        file.read(&mut buffer).unwrap();
        return u8s_to_u16(&buffer);
    }

    /**
     * 读取 interface
     */
    pub fn get_interface(cnt: u16, file: &mut File) -> Vec<u16> {
        let mut v: Vec<u16> = Vec::new();
        for j in 0..cnt {
            let mut buffer = [0u8; 2];
            file.read(&mut buffer).unwrap();
            let interface: u16 = u8s_to_u16(&buffer);
            v.push(interface);
        }
        return v;
    }

    pub fn get_field_count(file: &mut File) -> u16 {
        let mut buffer = [0u8; 2];
        file.read(&mut buffer).unwrap();
        return u8s_to_u16(&buffer);
    }

    pub fn get_field(cnt: u16, file: &mut File) -> Vec<FieldInfo> {
        let v: Vec<FieldInfo> = Vec::new();
        for j in 0..cnt {
            let mut f: FieldInfo = FieldInfo {
                access_flag: read_u16(file),
                name_index: read_u16(file),
                descriptor_index: read_u16(file),
                attribute_count: read_u16(file),
                atrributes: Vec::new(),
                value: Vec::new(),
            };
            f.atrributes = get_attribute(f.attribute_count, file);
        }
        return v;
    }

    pub fn get_method_count(file: &mut File) -> u16 {
        return read_u16(file);
    }

    pub fn get_method(cnt: u16, file: &mut File) -> Vec<MethodInfo> {
        let mut v: Vec<MethodInfo> = Vec::new();
        for j in 0..cnt {
            let mut m: MethodInfo = MethodInfo {
                access_flag: read_u16(file),
                name_index: read_u16(file),
                descriptor_index: read_u16(file),
                attributes_count: read_u16(file),
                attributes: Vec::new(),
                param: Vec::new(),
            };
            m.attributes = get_attribute(m.attributes_count, file);
            v.push(m);
        }
        return v;
    }

    pub fn get_attribute_count(file: &mut File) -> u16 {
        return read_u16(file);
    }

    pub fn get_attribute(cnt: u16, file: &mut File) -> Vec<AttributeInfo> {
        let mut ans: Vec<AttributeInfo> = Vec::new();
        for i in 0..cnt {
            let mut attribute_info: AttributeInfo = AttributeInfo {
                attribute_name_index: read_u16(file),
                attribute_length: read_u32(file),
                info: Vec::new(),
            };
            for j in 0..attribute_info.attribute_length {
                let mut buffer = [0u8; 1];
                file.read(&mut buffer).unwrap();
                attribute_info.info.push(buffer[0]);
            }
            ans.push(attribute_info);
        }
        return ans;
    }



}
