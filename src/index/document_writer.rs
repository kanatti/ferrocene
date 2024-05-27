use std::{collections::HashMap, rc::Rc};

use crate::{
    analysis::Analyzer,
    document::Document,
    store::{Directory, InputStream, OutputStream},
};

use super::{field_info::FieldInfos, fields_writer::FieldsWriter, Posting, Term};

pub const MAX_FIELD_LENGTH: usize = 1024;

pub struct PostingsTable {
    pub table: HashMap<Rc<Term>, Posting>,
}

impl PostingsTable {
    pub fn new() -> Self {
        Self {
            table: HashMap::new(),
        }
    }

    pub fn add(&mut self, term: Rc<Term>, position: usize) {
        let term_exists = self.table.contains_key(&term);

        if term_exists {
            let term = self.table.get_mut(&term).unwrap();
            term.positions.push(position as u32);
            term.freq += 1;
        } else {
            self.table
                .insert(term.clone(), Posting::new(term.clone(), position as u32));
        }
    }

    pub fn clear(&mut self) {
        self.table.clear();
    }
}

pub struct DocumentWriter<A, I, O, D>
where
    A: Analyzer,
    I: InputStream,
    O: OutputStream,
    D: Directory<Input = I, Output = O>,
{
    pub analyzer: A,
    pub directory: D,
    pub max_field_length: usize,
    pub field_infos: FieldInfos,
    pub postings_table: PostingsTable,
    pub field_lengths: Vec<usize>,
    pub field_boosts: Vec<f32>,
}

impl<A, I, O, D> DocumentWriter<A, I, O, D>
where
    A: Analyzer,
    I: InputStream,
    O: OutputStream,
    D: Directory<Input = I, Output = O>,
{
    pub fn new(analyzer: A, directory: D) -> Self {
        Self {
            analyzer,
            directory,
            max_field_length: MAX_FIELD_LENGTH,
            field_infos: FieldInfos::new(),
            postings_table: PostingsTable::new(),
            field_lengths: Vec::new(),
            field_boosts: Vec::new(),
        }
    }

    pub fn add_doc(&mut self, segment_id: &str, doc: Document) {
        // Write field names
        self.field_infos.add_doc(&doc);
        self.field_infos
            .write(&self.directory, &format!("{}.fnm", segment_id));

        // Write field values
        let mut fields_writer = FieldsWriter::new(&self.directory, segment_id, &self.field_infos);
        fields_writer.add_doc(&doc);
        drop(fields_writer);

        // Invert doc into postingTable
        self.postings_table.clear();
        self.field_lengths = vec![0; self.field_infos.size()];
        self.field_boosts = vec![1.0; self.field_infos.size()];

        self.invert_doc(&doc);

        // Sort postingTable into an array for postings

        // Write postings

        // Write norms of indexed fields
    }

    fn invert_doc(&mut self, doc: &Document) {
        for field in doc.fields.iter() {
            let field_name = &field.name;
            let field_value = &field.value;
            let field_number = self.field_infos.get_field_number(&field_name).unwrap();

            if !field.is_indexed {
                continue;
            }

            let mut position = self.field_lengths[field_number as usize];

            if !field.is_analyzed {
                let term = Rc::new(Term::new(field_name.to_owned(), field_value.to_owned()));
                self.postings_table.add(term, position);
                position += 1;
            } else {
                for token in self.analyzer.analyze(&field_value) {
                    let term = Rc::new(Term::new(field_name.to_owned(), token.to_owned()));

                    self.postings_table.add(term, position);

                    position += 1;
                    if position > self.max_field_length {
                        break;
                    }
                }
            }

            self.field_lengths[field_number as usize] = position;
            self.field_boosts[field_number as usize] *= field.boost;
        }
    }
}
