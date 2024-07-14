pub mod array {
    use log::info;

    use crate::common::{param::DataType, value::StackFrameValue};

    #[derive(Debug, Clone)]
    pub struct Array {
        pub id: u64,
        pub len: u32,
        pub array_type: DataType,
        pub data: Vec<StackFrameValue>,
    }

    impl Array {
        pub fn new(id: u64, len: u32, array_type: DataType) -> Array {
            let mut array = Array {
                id,
                len,
                array_type: array_type.clone(),
                data: Vec::new(),
            };

            match array_type {
                DataType::Array {
                    element_type,
                    depth,
                } => {
                    let array_type0 = *element_type;
                    if array_type0 == DataType::Char
                        || array_type0 == DataType::Short
                        || array_type0 == DataType::Int
                        || array_type0 == DataType::Long
                        || array_type0 == DataType::Byte
                        || array_type0 == DataType::Float
                        || array_type0 == DataType::Double
                    {
                        for _i in 0..len {
                            array.data.push(StackFrameValue::Int(0));
                        }
                    } else {
                        for _i in 0..len {
                            array.data.push(StackFrameValue::Null);
                        }
                    }
                }
                _ => panic!(),
            }

            array
        }
    }
}
