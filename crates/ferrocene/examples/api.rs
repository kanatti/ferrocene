use ferrocene::{
    document::Document,
    field::{Field, Store},
    index::{IndexWriter, IndexWriterConfig},
};

fn main() {
    let index_writer_config = IndexWriterConfig::with_defaults();
    let mut index_writer = IndexWriter {
        config: index_writer_config,
    };

    let mut doc1 = Document::new();
    doc1.add(Field::new(
        "filename".to_string(),
        "README.md".to_string(),
        Store::Yes,
    ));
    doc1.add(Field::new(
        "contents".to_string(),
        "Ferrocene is a search library inspired by lucene, written in rust".to_string(),
        Store::Yes,
    ));

    let mut doc2 = Document::new();
    doc2.add(Field::new(
        "filename".to_string(),
        "DEVELOPMENT.md".to_string(),
        Store::Yes,
    ));
    doc2.add(Field::new(
        "contents".to_string(),
        "You can use standard rust tools to work with project".to_string(),
        Store::Yes,
    ));

    index_writer.add(doc1);
    index_writer.add(doc2);

    index_writer.close();
}
