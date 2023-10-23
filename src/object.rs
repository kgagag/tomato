pub mod object{
    use std::collections::HashMap;
    #[derive(Debug, Clone)]
    pub struct Object {
        pub id: u64,
        pub class: usize,
        pub lock: u8,
        pub field: HashMap<u16, Vec<u8>>,
    }

    impl Object {
        pub fn new(id: u64, class: usize) -> Object {
            Object {
                id,
                class,
                lock: 0,
                field: HashMap::new(),
            }
        }
    }

}