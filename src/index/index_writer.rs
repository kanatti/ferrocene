use std::path::Path;

use crate::{analysis::Analyzer, document::{self, Document}};

pub struct IndexWriter<'p, A: Analyzer> {
    path: &'p Path,
    analyzer: A,
}

impl<'p, A: Analyzer> IndexWriter<'p, A> {
    pub fn new(path: &'p Path, analyzer: A) -> Self {
        Self { path, analyzer }
    }

    pub fn optimize(&mut self) {
        todo!()
    }

    pub fn close(&mut self) {
        todo!()
    }

    pub fn add_document(&mut self, document: Document) {
        println!("{:?}", document);
    }
}
