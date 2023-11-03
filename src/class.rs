pub mod class {
    use crate::param::param::MethodParameter;
    #[derive(Debug, Clone)]
    pub struct Class {
        pub magic: u32,
        pub minor_version: u16,
        pub major_version: u16,
        pub constant_pool_count: u16,
        pub constant_pool: Vec<ConstantPoolInfo>,
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
        pub class_name:String
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
                class_name:String::new()
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
        pub class_name:String,
        pub method_name:String,
        pub descriptor:String
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
                code
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

}
