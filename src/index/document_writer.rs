use std::collections::HashMap;

use crate::{analysis::Analyzer, document::Document, store::Directory};

use super::field_info::FieldInfos;

pub const MAX_FIELD_LENGTH: usize = 1024;

pub struct DocumentWriter<A: Analyzer, D: Directory>  {
    pub analyzer: A,
    pub directory: D,
    pub max_field_length: usize,
    pub field_infos: FieldInfos,
    pub postings_table: HashMap<String, String>,
    pub field_lengths: Vec<usize>,
    pub field_boosts: Vec<f32>,
}

impl <A: Analyzer, D: Directory> DocumentWriter<A, D> {
    pub fn new(analyzer: A, directory: D) -> Self {
        Self {
            analyzer,
            directory,
            max_field_length: MAX_FIELD_LENGTH,
            field_infos: FieldInfos::new(),
            postings_table: HashMap::new(),
            field_lengths: Vec::new(),
            field_boosts: Vec::new(),
        }
    }

    pub fn add_doc(&mut self, segment_id: &str, doc: Document) {
        // Write field names
        self.field_infos.add_doc(&doc);
        // TODO: Flush field info to segment_id + "fnm"

        // Write field values

        // Invert doc into postingTable

        // Sort postingTable into an array for postings

        // Write postings

        // Write norms of indexed fields
    }
}