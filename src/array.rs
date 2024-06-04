pub mod array{
    use crate::{param::param::DataType, value::value::StackFrameValue};
    #[derive(Debug, Clone)]
    pub struct Array {
        pub id: u64,
        pub len: u32,
        pub array_type: DataType,
        pub data:Vec<StackFrameValue> 
    }
    
    impl Array {
        pub fn new(id: u64, len: u32, array_type: DataType) -> Array{
           let mut array =  Array {
                id,
                len,
                array_type,
                data: Vec::new()
            };
            for _i in 0.. len {
                array.data.push(StackFrameValue::Byte(0));
            }
            array
        }
        
    }
}