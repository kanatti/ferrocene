/// A field is a section of a Document. Each field has two parts - name and value.
/// - Values are analyzed into terms.
/// - Keywords are not analyzed.
/// - Fields maybe stored in the index to be returned with document.
#[derive(Debug)]
pub struct Field {
    pub name: String,
    pub value: String,

    /// Whether the field is stored in the index.
    /// Stored field can be returned with document.
    pub is_stored: bool,

    /// Whether the field is indexed in the index.
    /// Indexed field can be searched.
    pub is_indexed: bool,

    /// Whether the field is analyzed.
    /// Useful for free text searching.
    pub is_analyzed: bool,

    /// Boost factor when scoring.
    pub boost: f32,
}

impl Field {
    /// Create a new field, which is index and analyzed but not stored.
    pub fn new(name: String, value: String) -> Field {
        Field {
            name,
            value,
            is_stored: false,
            is_indexed: true,
            is_analyzed: true,
            boost: 1.0,
        }
    }

    /// Create a keyword field, which is not analyzed but is stored and indexed.
    pub fn keyword(name: String, value: String) -> Field {
        Field {
            name,
            value,
            is_stored: true,
            is_indexed: true,
            is_analyzed: false,
            boost: 1.0,
        }
    }

    // Create a text field, which is stored, indexed and analyzed.
    pub fn text(name: String, value: String) -> Field {
        Field {
            name,
            value,
            is_stored: true,
            is_indexed: true,
            is_analyzed: true,
            boost: 1.0,
        }
    }

    /// Create an un-indexed field.
    /// Field is only stored and can be returned with doc, but cannot be searched.
    pub fn unindexed(name: String, value: String) -> Field {
        Field {
            name,
            value,
            is_stored: true,
            is_indexed: false,
            is_analyzed: false,
            boost: 1.0,
        }
    }
}

use std::fmt;
impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Conditional checks on is_stored, is_indexed and is_analyzed
        // to print the correct field type.
        if self.is_stored && self.is_indexed && !self.is_analyzed {
            write!(f, "Keyword<{}:{}>", self.name, self.value)
        } else if self.is_stored && self.is_indexed && self.is_analyzed {
            write!(f, "Text<{}:{}>", self.name, self.value)
        } else if self.is_stored && !self.is_indexed && !self.is_analyzed {
            write!(f, "UnIndexed<{}:{}>", self.name, self.value)
        } else {
            write!(f, "Field<{}:{}>", self.name, self.value)
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field() {
        let field = Field::new("name".to_string(), "John".to_string());
        assert_eq!(field.name, "name");
        assert_eq!(field.value, "John");
        assert_eq!(field.is_stored, false);
        assert_eq!(field.is_indexed, true);
        assert_eq!(field.is_analyzed, true);
        assert_eq!(field.boost, 1.0);        
    }

    #[test]
    fn test_keyword() {
        let field = Field::keyword("id".to_string(), "test123".to_string());
        assert_eq!(field.name, "id");
        assert_eq!(field.value, "test123");
        assert_eq!(field.is_stored, true);
        assert_eq!(field.is_indexed, true);
        assert_eq!(field.is_analyzed, false);
        assert_eq!(field.boost, 1.0);
    }

    #[test]
    fn test_text() {
        let field = Field::text("description".to_string(), "Hello World".to_string());
        assert_eq!(field.name, "description");
        assert_eq!(field.value, "Hello World");
        assert_eq!(field.is_stored, true);
        assert_eq!(field.is_indexed, true);
        assert_eq!(field.is_analyzed, true);
        assert_eq!(field.boost, 1.0);
    }

    #[test]
    fn test_unindexed() {
        let field = Field::unindexed("email".to_string(), "john@example.com".to_string());
        assert_eq!(field.name, "email");
        assert_eq!(field.value, "john@example.com");
        assert_eq!(field.is_stored, true);
        assert_eq!(field.is_indexed, false);
        assert_eq!(field.is_analyzed, false);
        assert_eq!(field.boost, 1.0);
    }

    #[test]
    fn test_display() {
        let keyword = Field::keyword("id".to_string(), "test123".to_string());
        let text = Field::text("description".to_string(), "Hello World".to_string());
        let unindexed = Field::unindexed("email".to_string(), "john@example.com".to_string());
        let field = Field::new("name".to_string(), "John".to_string());

        assert_eq!(keyword.to_string(), "Keyword<id:test123>");
        assert_eq!(text.to_string(), "Text<description:Hello World>");
        assert_eq!(unindexed.to_string(), "UnIndexed<email:john@example.com>");
        assert_eq!(field.to_string(), "Field<name:John>");
    }
}