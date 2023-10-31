use crate::field::Field;

pub struct Document {
    fields: Vec<Field>,
}

impl Document {
    pub fn new() -> Self {
        Document { fields: Vec::new() }
    }

    pub fn add(&mut self, field: Field) {
        self.fields.push(field);
    }
}
