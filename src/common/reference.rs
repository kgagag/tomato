pub mod reference{
    use crate::common::{array::array::Array, object::Object};

    #[derive(Debug, Clone)]
    pub enum Reference {
        Array(Array),
        Object(Object)
    }
}