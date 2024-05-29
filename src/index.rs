pub mod document_writer;
pub mod field_info;
pub mod fields_writer;
pub mod index_writer;

use std::rc::Rc;

pub use index_writer::{IndexWriter, IndexWriterConfig};

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Term {
    pub field: String,
    pub text: String,
}

impl Term {
    pub fn new(field: String, text: String) -> Self {
        Self { field, text }
    }
}


/// Information about a term in a doc
pub struct Posting {
    pub term: Rc<Term>,
    pub freq: u32,
    pub positions: Vec<u32>,
}

impl  Posting {
    pub fn new(term: Rc<Term>, position: u32) -> Self {
        Self { term, freq: 1, positions: vec![position] }
    }
}