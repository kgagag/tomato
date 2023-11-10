pub mod class_loader {
    //文件名（需要在main中先声明）+Mod名+引入对象()
    use crate::param::param::MethodParameter;
    use crate::class::Class;
    use crate::class::ConstantPoolInfo;
    use std::collections::HashMap;
    use std::fs::{self, File};
    use std::io;
    use std::io::Cursor;
    use crate::u8c::u8s_to_u16;
    use crate::u8c::u8s_to_u32;
    use crate::class::FieldInfo;
    use crate::class::MethodInfo;
    use crate::class::AttributeInfo;
    use byteorder::{BigEndian, ReadBytesExt};
    use log::info;
    use log::warn;
    use std::io::Read;
    use crate::runtime_data_area::add_method;


    fn parse_method_descriptor(
        descriptor: &Vec<u8>,
    ) -> Result<Option<Vec<MethodParameter>>, String> {
        let mut index = 0;
        let descriptor_length = descriptor.len();
        let mut parameters: Vec<MethodParameter> = Vec::new();
        while index < descriptor_length {
            let descriptor_char = descriptor[index] as char;
            if descriptor_char == '('{
                index += 1;
                continue;
            }
            if descriptor_char == ')'{
                break;
            }
            info!("descriptor_char:{:?}",&descriptor_char);
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
                    index = index + 1;
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
                        _ => {
                            warn!("Unknown array element type:{:?}",descriptor_char);
                            return Err("Unknown array element type".to_string())
                        }
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
        let class_path = String::from("D:/tomato/test/");
        let appendix = String::from(".class");
        let mut path = class_path + &name + &appendix;
        let rt_path = String::from("D:/tomato/rt/") + &name + &appendix;
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
        let mut buffer = Vec::new();
        // 从文件中读取数据并填充到 buffer 中
        let _ = file.read_to_end(&mut buffer);
        let mut cursor = io::Cursor::new(buffer);
        let mut class: Class = Class::new();
        class.magic = get_magic(&mut cursor);
        class.minor_version = get_minor_version(&mut cursor);
        class.major_version = get_major_version(&mut cursor);
        class.constant_pool_count = get_constant_pool_count(&mut cursor);
        class.constant_pool = read_constant_pool_info(class.constant_pool_count,&mut cursor);
        class.access_flags = get_access_flag(&mut cursor);
        class.this_class = get_this_class(&mut cursor);
        class.super_class = get_super_class(&mut cursor);
        class.interface_count = get_interface_count(&mut cursor);
        class.interfaces = get_interface(class.interface_count,&mut cursor);
        class.fields_count = get_field_count(&mut cursor);
        class.field_info = get_field(class.fields_count,&mut cursor);
        class.methods_count = get_method_count(&mut cursor);
        class.method_info = get_method(class.methods_count, &mut cursor);
        class.attributes_count = get_attribute_count(&mut cursor);
        class.attribute_info = get_attribute(class.attributes_count,&mut cursor);
        let this_class = class.constant_pool.get(&class.this_class).unwrap(); 
        info!("this_class:{:?}",this_class);
        match this_class {
            ConstantPoolInfo::Class(name_index) =>{
                info!("name_index:{:?}",name_index);
                let class_name =  class.constant_pool.get(name_index).unwrap();
                info!("class_name:{:?}",class_name);
                match class_name {
                    ConstantPoolInfo::Utf8(name) =>{
                        class.class_name = name.clone();
                    }
                    _=> panic!("wrong constant data type")
                }
    
            }
            _=> panic!("wrong constant data type")
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
                ConstantPoolInfo::Utf8(str) =>{
                    let method_name = class.constant_pool.get(&method_info.name_index).unwrap();
                    match method_name {
                        ConstantPoolInfo::Utf8(name) =>{
                            method_info.method_name = name.clone();
                        }
                        _=> panic!("wrong constant data type")
                    }
                    method_info.descriptor = str.clone();
                    info!("method_info.descripto:{:?}",&method_info.descriptor);
                    let result = parse_method_descriptor(&(str.clone().into_bytes()));
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
                _=> panic!("wrong constant data type")
            }            
        }
        return class;
    }

      /**
     * 获取魔数
     */
    pub fn get_magic(cursor:&mut Cursor<Vec<u8>>) -> u32 {
        // let mut buffer = [0u8; 4];
        //file.read(&mut buffer).unwrap();
        //let magic = cursor.read_u32::<BigEndian>().unwrap();
        //return u8s_to_u32(&buffer);
        return  cursor.read_u32::<BigEndian>().unwrap();
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
    pub fn get_minor_version(cursor:&mut Cursor<Vec<u8>>) -> u16 {
        return  cursor.read_u16::<BigEndian>().unwrap();
    }

    /**
     * 获取主版本号
     */
    pub fn get_major_version(cursor:&mut Cursor<Vec<u8>>) -> u16 {
        return  cursor.read_u16::<BigEndian>().unwrap();
    }

    /**
     * 获取常量池大小
     */

    pub fn get_constant_pool_count(cursor:&mut Cursor<Vec<u8>>) -> u16 {
        return  cursor.read_u16::<BigEndian>().unwrap();
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
    pub fn get_access_flag(cursor:&mut Cursor<Vec<u8>>) -> u16 {
        return  cursor.read_u16::<BigEndian>().unwrap();
    }

    /**
     * 读取 this_class
     */
    pub fn get_this_class(cursor:&mut Cursor<Vec<u8>>) -> u16 {
        return  cursor.read_u16::<BigEndian>().unwrap();
    }

    /**
     * 读取 super_class
     */
    pub fn get_super_class(cursor:&mut Cursor<Vec<u8>>) -> u16 {
        return  cursor.read_u16::<BigEndian>().unwrap();
    }

    /**
     * 读取 interface_count
     */
    pub fn get_interface_count(cursor:&mut Cursor<Vec<u8>>) -> u16 {
        return  cursor.read_u16::<BigEndian>().unwrap();

    }

    /**
     * 读取 interface
     */
    pub fn get_interface(cnt: u16, cursor:&mut Cursor<Vec<u8>>) -> Vec<u16> {
        let mut v: Vec<u16> = Vec::new();
        for j in 0..cnt {
            let mut buffer = [0u8; 2];
            let interface: u16 = cursor.read_u16::<BigEndian>().unwrap();
            v.push(interface);
        }
        return v;
    }

    pub fn get_field_count( cursor:&mut Cursor<Vec<u8>>) -> u16 {
        return cursor.read_u16::<BigEndian>().unwrap();
    }

    pub fn get_field(cnt: u16,cursor:&mut Cursor<Vec<u8>>) -> Vec<FieldInfo> {
        let v: Vec<FieldInfo> = Vec::new();
        for j in 0..cnt {
            let mut f: FieldInfo = FieldInfo {
                access_flag: cursor.read_u16::<BigEndian>().unwrap(),
                name_index: cursor.read_u16::<BigEndian>().unwrap(),
                descriptor_index: cursor.read_u16::<BigEndian>().unwrap(),
                attribute_count:  cursor.read_u16::<BigEndian>().unwrap(),
                atrributes: Vec::new(),
                value: Vec::new(),
            };
            f.atrributes = get_attribute(f.attribute_count, cursor);
        }
        return v;
    }

    pub fn get_method_count(cursor:&mut Cursor<Vec<u8>>) -> u16 {
        return cursor.read_u16::<BigEndian>().unwrap();
    }

    pub fn get_method(cnt: u16, cursor:&mut Cursor<Vec<u8>>) -> Vec<MethodInfo> {
        let mut v: Vec<MethodInfo> = Vec::new();
        for j in 0..cnt {
            let mut m: MethodInfo = MethodInfo {
                access_flag: cursor.read_u16::<BigEndian>().unwrap(),
                name_index: cursor.read_u16::<BigEndian>().unwrap(),
                descriptor_index: cursor.read_u16::<BigEndian>().unwrap(),
                attributes_count: cursor.read_u16::<BigEndian>().unwrap(),
                attributes: Vec::new(),
                param: Vec::new(),
                class_name:String::new(),
                method_name:String::new(),
                descriptor:String::new()
            };
            m.attributes = get_attribute(m.attributes_count, cursor);
            v.push(m);
        }
        return v;
    }

    pub fn get_attribute_count(cursor:&mut Cursor<Vec<u8>>) -> u16 {
        return cursor.read_u16::<BigEndian>().unwrap();
    }

    pub fn get_attribute(cnt: u16, cursor:&mut Cursor<Vec<u8>>) -> Vec<AttributeInfo> {
        let mut ans: Vec<AttributeInfo> = Vec::new();
        for i in 0..cnt {
            let mut attribute_info: AttributeInfo = AttributeInfo {
                attribute_name_index: cursor.read_u16::<BigEndian>().unwrap(),
                attribute_length: cursor.read_u32::<BigEndian>().unwrap(),
                info: Vec::new(),
            };
            for j in 0..attribute_info.attribute_length {
                attribute_info.info.push(cursor.read_u8().unwrap());
            }
            ans.push(attribute_info);
        }
        return ans;
    }



fn read_constant_pool_info<R: Read>(mut constant_pool_count:u16,reader: &mut R) -> HashMap<u16,ConstantPoolInfo> {
    let mut constant_pool = HashMap::new();

    let mut index:u16 = 1;
    while constant_pool_count - 1 > 0 {
        let tag = reader.read_u8().expect("Failed to read constant tag");
        info!("constant_pool_tag:{:?}",tag);
        match tag {
            1 => {
                let length = reader.read_u16::<BigEndian>().expect("Failed to read UTF-8 length");
                let mut data = vec![0; length as usize];
                reader.read_exact(&mut data).expect("Failed to read UTF-8 data");
                let utf8_string = String::from_utf8(data).expect("Invalid UTF-8 string");
                constant_pool.insert(index,ConstantPoolInfo::Utf8(utf8_string));
                index += 1;
            }
            3 =>{
                 constant_pool.insert(index,ConstantPoolInfo::Integer(reader.read_i32::<BigEndian>().expect("Failed to read integer")));
                index += 1;    
            }
            4 =>{
                constant_pool.insert(index,ConstantPoolInfo::Float(reader.read_f32::<BigEndian>().expect("Failed to read float")));
                index += 1;
            }
            5 => {
                constant_pool.insert(index,ConstantPoolInfo::Long(reader.read_i64::<BigEndian>().expect("Failed to read long")));
                constant_pool_count = constant_pool_count - 1;
                index += 2;
            }
            6 => {
                    constant_pool.insert(index,ConstantPoolInfo::Double(reader.read_f64::<BigEndian>().expect("Failed to read double")));
                    constant_pool_count = constant_pool_count - 1;
                    index += 2;
                }
            7 =>{ 
                constant_pool.insert(index,ConstantPoolInfo::Class(reader.read_u16::<BigEndian>().expect("Failed to read class index")));
                index += 1;
            }
            8 =>{ 
                constant_pool.insert(index,ConstantPoolInfo::String(reader.read_u16::<BigEndian>().expect("Failed to read string index")));
                index += 1;
            }
            9 => {
                let class_index = reader.read_u16::<BigEndian>().expect("Failed to read fieldref class index");
                let name_and_type_index = reader.read_u16::<BigEndian>().expect("Failed to read fieldref name and type index");
                constant_pool.insert(index,ConstantPoolInfo::Fieldref(class_index, name_and_type_index));
                index += 1;
            }
            10 => {
                let class_index = reader.read_u16::<BigEndian>().expect("Failed to read methodref class index");
                let name_and_type_index = reader.read_u16::<BigEndian>().expect("Failed to read methodref name and type index");
                constant_pool.insert(index,ConstantPoolInfo::Methodref(class_index, name_and_type_index));
                index += 1;
            }
            11 => {
                let class_index = reader.read_u16::<BigEndian>().expect("Failed to read interface methodref class index");
                let name_and_type_index = reader.read_u16::<BigEndian>().expect("Failed to read interface methodref name and type index");
                constant_pool.insert(index,ConstantPoolInfo::InterfaceMethodref(class_index, name_and_type_index));
            }
            12 => {
                let name_index = reader.read_u16::<BigEndian>().expect("Failed to read name and type name index");
                let descriptor_index = reader.read_u16::<BigEndian>().expect("Failed to read name and type descriptor index");
                constant_pool.insert(index,ConstantPoolInfo::NameAndType(name_index, descriptor_index));
                index += 1;
            }
            15 => {
                let reference_kind = reader.read_u8().expect("Failed to read method handle reference kind");
                let reference_index = reader.read_u16::<BigEndian>().expect("Failed to read method handle reference index");
                constant_pool.insert(index,ConstantPoolInfo::MethodHandle(reference_kind, reference_index));
                index += 1;
            }
            16 => {
                constant_pool.insert(index,ConstantPoolInfo::MethodType(reader.read_u16::<BigEndian>().expect("Failed to read method type descriptor index")));
                index += 1;
            }
            18 => {
                let bootstrap_method_attr_index = reader.read_u16::<BigEndian>().expect("Failed to read invoke dynamic bootstrap method attribute index");
                let name_and_type_index = reader.read_u16::<BigEndian>().expect("Failed to read invoke dynamic name and type index");
                constant_pool.insert(index,ConstantPoolInfo::InvokeDynamic(bootstrap_method_attr_index, name_and_type_index));
                index += 1;
            }
            19 => {
                constant_pool.insert(index,ConstantPoolInfo::Module(reader.read_u16::<BigEndian>().expect("Failed to read module index")));
                index += 1;
            }
            20 => {
                constant_pool.insert(index,ConstantPoolInfo::Package(reader.read_u16::<BigEndian>().expect("Failed to read package index")));
                index += 1;
            }
            21 => {
                let method_handle_reference_kind = reader.read_u8().expect("Failed to read method handle reference kind");
                let method_handle_reference_index = reader.read_u16::<BigEndian>().expect("Failed to read method handle reference index");
                constant_pool.insert(index,ConstantPoolInfo::MethodPointer(method_handle_reference_kind, method_handle_reference_index));
                index += 1;
            }
            22 => {
                let invoke_static_dynamic_name_and_type_index = reader.read_u16::<BigEndian>().expect("Failed to read invoke static dynamic name and type index");
                let invoke_static_dynamic_bootstrap_method_attr_index = reader.read_u16::<BigEndian>().expect("Failed to read invoke static dynamic bootstrap method attribute index");
                constant_pool.insert(index,ConstantPoolInfo::InvokeStaticDynamic(invoke_static_dynamic_bootstrap_method_attr_index, invoke_static_dynamic_name_and_type_index));
                index += 1;
            }
            23 => {
                let bootstrap_method_attr_index = reader.read_u16::<BigEndian>().expect("Failed to read bootstrap method attribute index");
                let name_and_type_index = reader.read_u16::<BigEndian>().expect("Failed to read bootstrap method name and type index");
                constant_pool.insert(index,ConstantPoolInfo::BootstrapMethod(bootstrap_method_attr_index, name_and_type_index));
                index += 1;
            }
            24 => {
                let method_type_reference_index = reader.read_u16::<BigEndian>().expect("Failed to read method type reference index");
                constant_pool.insert(index,ConstantPoolInfo::MethodTypeReference(method_type_reference_index));
                index += 1;
            }
            // 添加更多常量类型的处理
            _ => panic!("Invalid constant pool tag: {}", tag)
        }
        //index += 1;
        constant_pool_count = constant_pool_count - 1;
    }

    constant_pool
}

}

