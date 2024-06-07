use std::{collections::HashMap, rc::Rc};

use crate::{
    analysis::Analyzer,
    document::Document,
    store::{
        fs_directory::{FSInputStream, FSOutputStream},
        Directory, FSDirectory, InputStream, OutputStream,
    },
};

use super::{field_info::FieldInfos, fields_writer::FieldsWriter, Posting, Term};

pub const MAX_FIELD_LENGTH: usize = 1024;

pub type FSDocumentWriter<A> = DocumentWriter<A, FSInputStream, FSOutputStream, FSDirectory>;

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

    pub fn values(&self) -> Vec<&Posting> {
        self.table.values().collect()
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
        let mut postings = self.postings_table.values();
        postings.sort_by_key(|p| &p.term);

        // Write postings
        println!("{:?}", postings);

        // Write norms of indexed fields
    }

    fn invert_doc(&mut self, doc: &Document) {
        for field in doc.fields.iter() {
            // Skip non-indexed fields
            if !field.is_indexed {
                continue;
            }

            let field_name = &field.name;
            let field_value = &field.value;
            let field_number = self.field_infos.get_field_number(&field_name).unwrap();
            let mut position = self.field_lengths[field_number as usize];

            // No need to run analyzer for non-analyzed fields.
            // Whole field is stored as a single term.
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

    fn _write_postings(&mut self, segment_id: &str, postings: &Vec<&Posting>) {
        let mut _freq = self.directory.create_file(&format!("{}.frq", segment_id)).unwrap();
        let mut _pos = self.directory.create_file(&format!("{}.prx", segment_id)).unwrap();

        // TermInfosWriter
        // TermInfo

        // Look through postings
        for _posting in postings {
            // Add an entry to the dictionary with pointers to prox and freq files.

            // Add an entry to the freq file.

            // Write positiongs using delta-encoding.
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{analysis::StandardAnalyzer, document::Field, store::FSDirectory};

    use super::*;

    #[test]
    fn test_postings_table() {
        let mut postings_table = PostingsTable::new();

        let term1 = Rc::new(Term::new("title".to_owned(), "Tests".to_owned()));
        let term2 = Rc::new(Term::new("description".to_owned(), "unit".to_owned()));
        let term3 = Rc::new(Term::new("description".to_owned(), "tests".to_owned()));
        let term4 = Rc::new(Term::new("description".to_owned(), "and".to_owned()));
        let term5 = Rc::new(Term::new(
            "description".to_owned(),
            "integration".to_owned(),
        ));
        let term6 = Rc::new(Term::new("description".to_owned(), "tests".to_owned()));

        postings_table.add(term1.clone(), 0);
        postings_table.add(term2.clone(), 0);
        postings_table.add(term3.clone(), 1);
        postings_table.add(term4.clone(), 2);
        postings_table.add(term5.clone(), 3);
        postings_table.add(term6.clone(), 4);

        assert_eq!(postings_table.table.len(), 5);

        assert_eq!(postings_table.table.get(&term1).unwrap().freq, 1);
        assert_eq!(postings_table.table.get(&term1).unwrap().positions.len(), 1);
        assert_eq!(postings_table.table.get(&term1).unwrap().positions[0], 0);

        assert_eq!(postings_table.table.get(&term3).unwrap().freq, 2);
        assert_eq!(postings_table.table.get(&term3).unwrap().positions.len(), 2);
        assert_eq!(postings_table.table.get(&term3).unwrap().positions[0], 1);
        assert_eq!(postings_table.table.get(&term3).unwrap().positions[1], 4);
    }

    #[test]
    fn test_document_writer() {
        // Setup
        let root_dir = tempfile::tempdir().expect("Failed to create temp dir");

        let path = root_dir.path().join("test-index");
        let directory = FSDirectory::new(path).unwrap();
        let analyzer = StandardAnalyzer::new();

        let mut document_writer: FSDocumentWriter<StandardAnalyzer> =
            DocumentWriter::new(analyzer, directory);

        let mut doc = Document::new();
        doc.add(Field::keyword("title".to_owned(), "Tests".to_owned()));
        doc.add(Field::text(
            "description".to_owned(),
            "unit tests and integration tests".to_owned(),
        ));

        // Execute
        document_writer.add_doc("test-segment", doc);

        // Verify Field Infos
        let field_infos = document_writer.field_infos;
        assert_eq!(field_infos.size(), 2);
        assert_eq!(field_infos.get_field_name(0).unwrap(), "title");
        assert_eq!(field_infos.get_field_name(1).unwrap(), "description");

        // Verify Postings Table
        let postings_table = document_writer.postings_table;
        assert_eq!(postings_table.table.len(), 5);

        let term1 = Rc::new(Term::new("title".to_owned(), "Tests".to_owned()));
        assert_eq!(postings_table.table.get(&term1).unwrap().freq, 1);
        assert_eq!(postings_table.table.get(&term1).unwrap().positions.len(), 1);
        assert_eq!(postings_table.table.get(&term1).unwrap().positions[0], 0);

        let term2: Rc<Term> = Rc::new(Term::new("description".to_owned(), "tests".to_owned()));
        assert_eq!(postings_table.table.get(&term2).unwrap().freq, 2);
        assert_eq!(postings_table.table.get(&term2).unwrap().positions.len(), 2);
        assert_eq!(postings_table.table.get(&term2).unwrap().positions[0], 1);
        assert_eq!(postings_table.table.get(&term2).unwrap().positions[1], 4);

        // Verify files generated
    }
}
