use std::path::Path;

use crate::{analysis::Analyzer, document::{self, Document}};

pub struct IndexWriter<'p, A> {
    path: &'p Path,
    analyzer: A,
    write_mode: WriteMode
}

pub enum WriteMode {
    CREATE,
    OPEN
}

impl<'p, A: Analyzer> IndexWriter<'p, A> {
    pub fn create(path: &'p Path, analyzer: A) -> Self {
        Self { path, analyzer, write_mode: WriteMode::CREATE }
    }

    pub fn open(path: &'p Path, analyzer: A) -> Self {
        Self { path, analyzer, write_mode: WriteMode::OPEN }
    }

    pub fn add_document(&mut self, document: Document) {
        println!("{:?}", document);
    }

    pub fn optimize(&mut self) {
        todo!()
    }

    pub fn close(&mut self) {
        todo!()
    }
}

