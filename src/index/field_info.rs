use std::{collections::HashMap, rc::Rc};

use crate::{
    document::{Document, Field},
    store::{Directory, InputStream, OutputStream},
};

pub struct FieldInfo {
    pub name: String,
    pub is_indexed: bool,
    pub number: u32,
}

/// Stores all the field infos.
///
///
/// ```md
///   by_name                                                          
/// ┌─────────────┐          ┌──────────────────┐                        
/// │             │          │ name: user_id    │                        
/// │  user_id   ──────────▶ │ is_indexed: true │                        
/// │             │          │ number: 1        │                        
/// │             │          └──────────────────┘    ┌──────────────────┐
/// │             │                           ▲      │ name: user_name  │
/// │  user_name ─────────────────────────────│────▶ │ is_indexed: true │
/// │             │                           │      │ number: 2        │
/// │             │      ┌──────────────────┐ │      └──────────────────┘
/// │             │      │ name: title      │ │             ▲            
/// │  title     ──────▶ │ is_indexed: true │ │             │            
/// │             │      │ number: 0        │ │             │            
/// │             │      └──────────────────┘ │             │            
/// └─────────────┘               ▲           │             │            
///                               │           │             │            
///                           ┌──────────────────────────────────┐       
///                           │   0           1             2    │       
///                           └──────────────────────────────────┘       
///                                        by_number    
/// ```
pub struct FieldInfos {
    pub by_number: Vec<Rc<FieldInfo>>,
    pub by_name: HashMap<String, Rc<FieldInfo>>,
}

impl FieldInfos {
    /// Create empty FieldInfos.
    pub fn new() -> Self {
        FieldInfos {
            by_number: vec![],
            by_name: HashMap::new(),
        }
    }

    pub fn add_doc(&mut self, doc: &Document) {
        doc.fields.iter().for_each(|field| {
            self.add_field(field);
        })
    }

    pub fn add_other(&mut self, other: &FieldInfos) {
        other.by_number.iter().for_each(|fi| {
            self.add(fi.name.clone(), fi.is_indexed);
        })
    }

    pub fn add_field(&mut self, field: &Field) {
        self.add(field.name.clone(), field.is_indexed);
    }

    pub fn add(&mut self, name: String, is_indexed: bool) {
        if self.by_name.contains_key(&name) {
            return;
        }

        let field_info = Rc::new(FieldInfo {
            name: name.clone(),
            is_indexed,
            number: self.by_number.len() as u32,
        });
        self.by_number.push(field_info.clone());
        self.by_name.insert(name.clone(), field_info.clone());
    }

    pub fn get_field_number(&self, name: &str) -> Option<u32> {
        self.by_name.get(name).map(|fi| fi.number)
    }

    pub fn get_field_name(&self, number: u32) -> Option<String> {
        self.by_number
            .get(number as usize)
            .map(|fi| fi.name.clone())
    }

    pub fn get_field_info_by_name(&self, name: &str) -> Option<Rc<FieldInfo>> {
        self.by_name.get(name).cloned()
    }

    pub fn get_field_info_by_number(&self, number: u32) -> Option<Rc<FieldInfo>> {
        self.by_number.get(number as usize).cloned()
    }

    pub fn size(&self) -> usize {
        self.by_number.len()
    }

    pub fn read<I, O, D>(&mut self, dir: &D, filename: &str)
    where
        I: InputStream,
        O: OutputStream,
        D: Directory<Input = I, Output = O>,
    {
        let mut input = dir.open_file(filename).unwrap();

        let size = input.read_vint() as usize;

        for _i in 0..size {
            let name = input.read_string();
            let is_indexed = input.read_bool();

            self.add(name, is_indexed);
        }
    }

    pub fn write<I, O, D>(&self, dir: &D, filename: &str)
    where
        I: InputStream,
        O: OutputStream,
        D: Directory<Input = I, Output = O>,
    {
        let mut output = dir.create_file(filename).unwrap();

        output.write_vint(self.size() as u32);

        self.by_number.iter().for_each(|fi| {
            output.write_string(&fi.name);
            output.write_bool(fi.is_indexed);
        });
    }
}

#[cfg(test)]
mod tests {
    use crate::store::FSDirectory;

    use super::*;

    #[test]
    fn test_field_infos() {
        let mut field_infos = FieldInfos::new();

        field_infos.add("user_id".to_string(), true);
        field_infos.add("user_name".to_string(), true);
        field_infos.add("title".to_string(), true);

        assert_eq!(field_infos.get_field_number("user_id").unwrap(), 0);
        assert_eq!(field_infos.get_field_number("user_name").unwrap(), 1);
        assert_eq!(field_infos.get_field_number("title").unwrap(), 2);

        assert_eq!(field_infos.get_field_name(0).unwrap(), "user_id");
        assert_eq!(field_infos.get_field_name(1).unwrap(), "user_name");
        assert_eq!(field_infos.get_field_name(2).unwrap(), "title");

        let field = field_infos.get_field_info_by_name("user_id").unwrap();
        assert_eq!(field.name, "user_id");
        assert_eq!(field.is_indexed, true);
        assert_eq!(field.number, 0);
    }

    #[test]
    fn test_field_infos_io() {
        let mut field_infos = FieldInfos::new();

        field_infos.add("user_id".to_string(), true);
        field_infos.add("user_name".to_string(), true);
        field_infos.add("title".to_string(), true);

        let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
        let path = temp_dir.path().join("test-index");

        let directory = FSDirectory::new(&path).expect("Filed to create FSDirectory over temp dir");

        // Write field-infos to file
        field_infos.write(&directory, "field-infos.fnm");

        // Read field-infos from file
        let mut field_infos_read = FieldInfos::new();
        field_infos_read.read(&directory, "field-infos.fnm");

        assert_eq!(field_infos.get_field_number("user_id").unwrap(), 0);
        assert_eq!(field_infos.get_field_number("user_name").unwrap(), 1);
        assert_eq!(field_infos.get_field_number("title").unwrap(), 2);

        assert_eq!(field_infos.get_field_name(0).unwrap(), "user_id");
        assert_eq!(field_infos.get_field_name(1).unwrap(), "user_name");
        assert_eq!(field_infos.get_field_name(2).unwrap(), "title");

        temp_dir.close().expect("Failed to close temp dir");
    }
}
