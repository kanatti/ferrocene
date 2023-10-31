pub struct Field {
    name: String,
    value: String,
    store: Store,
}

pub enum Store {
    Yes,
    No,
}


impl Field {
    pub fn new(name: String, value: String, store: Store) -> Field {
        Field { name, value, store }
    }
}