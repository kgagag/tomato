pub mod array{
    use crate::{param::param::MethodParameter, value::value::StackFrameValue};
    #[derive(Debug, Clone)]
    pub struct Array {
        pub id: u32,
        pub len: u32,
        pub array_type: MethodParameter,
        pub data:Vec<StackFrameValue> ,
    }
    
    impl Array {
        pub fn new(id: u32, len: u32, array_type: MethodParameter) -> Array{
           let mut array =  Array {
                id,
                len,
                array_type,
                data: Vec::new(),
            };
            for i in 0.. len {
                array.data.push(StackFrameValue::Byte(0));
            }
            return array;
        }
        
    }
}