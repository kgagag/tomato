pub mod reference{
    use crate::array::array::Array;
    use crate::object::Object;
    #[derive(Debug, Clone)]
    pub enum Reference {
        Array(Array),
        Object(Object)
    }
}