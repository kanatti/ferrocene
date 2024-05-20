use super::Field;

/// A document is a collection of fields.
/// Its the unit of indexing and search.
pub struct Document {
    pub fields: Vec<Field>,
}

impl Document {
    /// A new document without any fields.
    pub fn new() -> Self {
        Document { fields: Vec::new() }
    }

    pub fn add(&mut self, field: Field) {
        self.fields.push(field);
    }

    pub fn get_field(&self, name: &str) -> Option<&Field> {
        self.fields.iter().find(|f| f.name == name)
    }

    pub fn get(&self, name: &str) -> Option<&str> {
        self.get_field(name).map(|f| f.value.as_str())
    }
}

use std::fmt::Display;
impl Display for Document {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Document<")?;
        write!(f, "{}", self.fields.iter().map(ToString::to_string).collect::<Vec<_>>().join(" "))?;
        write!(f, ">")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document() {
        let mut doc = Document::new();
        doc.add(Field::keyword("name".to_string(), "John".to_string()));
        doc.add(Field::new("age".to_string(), "30".to_string()));

        assert_eq!(doc.get("name"), Some("John"));
        assert_eq!(doc.get("age"), Some("30"));
        assert_eq!(doc.get("weight"), None);
    }

    #[test]
    fn test_display() {
        let mut doc = Document::new();
        doc.add(Field::keyword("name".to_string(), "John".to_string()));
        doc.add(Field::new("age".to_string(), "30".to_string()));

        assert_eq!(doc.to_string(), "Document<Keyword<name:John> Field<age:30>>");
    }
}
