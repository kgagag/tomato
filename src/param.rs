pub mod param{
    
    #[derive(Debug, Clone)]
    pub enum DataType {
        Byte,
        Char,
        Double,
        Float,
        Int,
        Long,
        Reference(String),
        Short,
        Boolean,
        Array {
            element_type: Box<DataType>,
            depth: u32,
        },
        Unknown
    }
}