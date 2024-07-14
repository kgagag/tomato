 
    #[derive(Debug, Clone)]
    #[derive(PartialEq)]
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
            depth: u8,
        },
        Unknown
    }