pub mod array{
    pub enum Type {
        Byte,
        Char,
        Double,
        Float,
        Int,
        Long,
        Reference(String),
        Short,
        Boolean
    }
}