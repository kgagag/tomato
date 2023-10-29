pub mod value {
    #[derive(Debug, Clone)]
    pub enum StackFrameValue {
        Byte(i8),
        Char(u16),
        Double(f64),
        Float(f32),
        Int(i32),
        Long(i64),
        Reference(u32),
        Short(i16),
        Boolean(bool),
        Array {
            element_type: Box<StackFrameValue>,
            depth: u32,
        },
    }
}