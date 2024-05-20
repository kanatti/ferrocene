use crate::{analysis::Analyzer, store::Directory};

pub struct DocumentWriter {
    pub analyzer: Box<dyn Analyzer>,
    pub directory: Box<dyn Directory>,
    pub max_field_length: usize,    
}