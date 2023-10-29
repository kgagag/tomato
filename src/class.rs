pub mod class {
    use crate::param::param::MethodParameter;
    #[derive(Debug, Clone)]
    pub struct Class {
        pub magic: u32,
        pub minor_version: u16,
        pub major_version: u16,
        pub constant_pool_count: u16,
        pub constant_pool: Vec<Vec<u8>>,
        pub access_flags: u16,
        pub this_class: u16,
        pub super_class: u16,
        pub interface_count: u16,
        pub interfaces: Vec<u16>,
        pub fields_count: u16,
        pub field_info: Vec<FieldInfo>,
        pub methods_count: u16,
        pub method_info: Vec<MethodInfo>,
        pub attributes_count: u16,
        pub attribute_info: Vec<AttributeInfo>,
        pub id: usize,
    }

    impl Class {
        pub fn new() -> Self {
            Self {
                magic: 0,
                minor_version: 0,
                major_version: 0,
                constant_pool_count: 0,
                constant_pool: Vec::new(),
                access_flags: 0,
                this_class: 0,
                super_class: 0,
                interface_count: 0,
                interfaces: Vec::new(),
                fields_count: 0,
                field_info: Vec::new(),
                methods_count: 0,
                method_info: Vec::new(),
                attributes_count: 0,
                attribute_info: Vec::new(),
                id: 0,
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct MethodInfo {
        pub access_flag: u16,
        pub name_index: u16,
        pub descriptor_index: u16,
        pub attributes_count: u16,
        pub param: Vec<MethodParameter>,
        pub attributes: Vec<AttributeInfo>,
    }

    #[derive(Debug, Clone)]
    pub struct FieldInfo {
        pub access_flag: u16,
        pub name_index: u16,
        pub descriptor_index: u16,
        pub attribute_count: u16,
        pub atrributes: Vec<AttributeInfo>,
        pub value: Vec<u8>,
    }

    pub struct ConstantMethod {
        tag: u8,
        class_index: u16,
        name_and_type_index: u16,
    }

    pub struct ConstantField {
        tag: u8,
        class_index: u16,
        name_and_type_index: u16,
    }

    pub struct ConstantClass {
        tag: u8,
        name_index: u16,
    }

    pub struct ConstantUtf8 {
        tag: u8,
        length: u16,
        bytes: Vec<u8>,
    }

    pub struct ConstantString {
        tag: u8,
        string_index: u16,
    }

    pub struct ConstantNameAndType {
        tag: u8,
        name_index: u16,
        descriptor_index: u16,
    }

    pub struct ConstantInterfaceMethod {
        tag: u8,
        class_index: u16,
        name_and_type_index: u16,
    }

    pub struct ConstantInteger {
        tag: u8,
        bytes: i32,
    }

    pub struct ConstantFloat {
        tag: u8,
        bytes: f32,
    }

    pub struct ConstantLong {
        tag: u8,
        bytes: i64,
    }

    pub struct ConstantDouble {
        tag: u8,
        bytes: f64,
    }

    pub struct ConstantMethodHandle {
        tag: u8,
        reference_kind: u8,
        reference_index: u16,
    }

    pub struct ConstantInvokeDynamic {
        tag: u8,
        bootstrap_method_attr_index: u16,
        name_and_type_index: u16,
    }

    #[derive(Debug, Clone)]
    pub struct AttributeInfo {
        pub attribute_name_index: u16,
        pub attribute_length: u32,
        pub info: Vec<u8>,
    }

    #[derive(Debug)]
    pub struct CodeAttribute {
        pub max_stack: u16,
        pub max_locals: u16,
        pub code_length: u32,
        pub code: Vec<u8>,
        // pub exception_table_length: u16,
        // pub exception_table: ExceptionTable,
        // pub attributes_count: u16,
        // pub attribute_info: AttributeInfo,
    }

    impl CodeAttribute {
        pub fn new(
            max_stack: u16,
            max_locals: u16,
            code_length: u32,
            code: Vec<u8>,
        ) -> CodeAttribute {
            CodeAttribute {
                max_stack,
                max_locals,
                code_length,
                code,
                // exception_table_length,
                // exception_table,
                // attributes_count,
                // attribute_info
            }
        }
    }

    pub struct AttributeConstantValue {
        attribute_name_index: u16,
        attribute_length: u32,
        constantvalue_index: u16,
    }

    #[derive(Debug)]
    pub struct ExceptionTable {
        start_pc: u16,
        end_pc: u16,
        handle_pc: u16,
        catch_type: u16,
    }

    pub struct AttributeCode {
        attribute_name_index: u16,
        attribute_length: u32,
        max_stack: u16,
        max_locals: u16,
        code_length: u32,
        code: Vec<u8>,
        exception_table_length: u16,
        exception_table: Vec<ExceptionTable>,
    }

    pub struct StackMapTable {
        attribute_name_index: u16,
        attribute_length: u32,
        number_of_entries: u16,
    }

    #[derive(Debug)]
    pub struct Exception {}
    /*
    fn parse_method_descriptor(descriptor: &Vec<u8>) -> Result<Option<Vec<MethodParameter>>, String> {
        let mut index = 0;
        let descriptor_length = descriptor.len();
        let mut parameters: Vec<MethodParameter> = Vec::new();
        let string = String::from_utf8_lossy(&descriptor);
        while index < descriptor_length {
            let descriptor_char = descriptor[index] as char;

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
                    while index < descriptor_length && descriptor[index] as char == '[' {
                        array_depth += 1;
                        index += 1;
                    }
                    if index >= descriptor_length {
                        return Err("Invalid array parameter descriptor".to_string());
                    }
                    let element_type = match descriptor[index] as char {
                        'B' => MethodParameter::Byte,
                        'C' => MethodParameter::Char,
                        'D' => MethodParameter::Double,
                        'F' => MethodParameter::Float,
                        'I' => MethodParameter::Int,
                        'J' => MethodParameter::Long,
                        'L' => {
                            // Handle reference type array parameters
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
               // _ => return Err(format!("Unknown parameter descriptor: {}", descriptor_char)),
               _ =>{

               }
            }

            index += 1;
        }

        if parameters.is_empty() {
            Ok(None) // No parameters, return None
        } else {
            Ok(Some(parameters))
        }
    }

    /***
     * 类加载
     */
    pub fn load_class(name: &String) -> Class {
        //设置class_path
        let class_path = String::from("E:/rustwork/tomato/test/");
        let appendix = String::from(".class");
        let mut path = class_path + &name + &appendix;
        let rt_path =  String::from("E:/rustwork/tomato/rt/") + &name+&appendix;
        match fs::metadata(&path) {
            Ok(metadata) => {
                if !metadata.is_file(){
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
    */
}
