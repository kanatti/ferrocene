use ferrocene::{
    document::{Document, Field},
    index::field_info::FieldInfos, store::FSDirectory,
};

fn main() {
    let mut field_infos = FieldInfos::new();

    let mut doc1 = Document::new();
    doc1.add(Field::keyword(
        "filename".to_string(),
        "README.md".to_string(),
    ));
    doc1.add(Field::text(
        "contents".to_string(),
        "Ferrocene is a search library inspired by lucene, written in rust".to_string(),
    ));

    field_infos.add_doc(&doc1);

    let workspace_path = std::env::current_dir().unwrap();
    let index_path = workspace_path.join("test-data").join("api");

    let directory = FSDirectory::new(index_path).unwrap();

    field_infos.write(&directory, "field-infos.fnm")
}
