pub mod param{
    
    #[derive(Debug, Clone)]
    pub enum MethodParameter {
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
            element_type: Box<MethodParameter>,
            depth: u32,
        },
    }
}