use std::{collections::HashMap, rc::Rc};

use crate::{document::{Document, Field}, store::Directory};

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

    /// Load FieldInfos from existing file.
    pub fn load(_dir: impl Directory, _filename: &str) {
        todo!()
    }

    pub fn flush(_dir: impl Directory, _filename: &str) {
        todo!()
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
        self.by_number.get(number as usize).map(|fi| fi.name.clone())
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
}