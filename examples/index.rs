use std::{fs::File, io::Read, path::Path};

use ferrocene::{
    analysis::StandardAnalyzer,
    document::{Document, Field},
    index::IndexWriter,
};

fn main() {
    let workspace_path = std::env::current_dir().unwrap();
    let index_path = workspace_path.join("examples").join("index1");
    let data_path = workspace_path.join("examples").join("data");

    // Create a new index writer
    let mut writer = IndexWriter::new(&index_path, StandardAnalyzer::new());

    // index all files in data_path
    for entry in std::fs::read_dir(data_path).unwrap() {
        let path = entry.unwrap().path();
        let document = file_document(&path);
        writer.add_document(document);
    }
}

fn file_document(path: &Path) -> Document {
    let file_name = path.to_str().unwrap().to_string();
    let mut file = File::open(path).unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();


    let mut document = Document::new();
    document.add(Field::keyword(
        "file_name".to_string(),
        file_name,
    ));

    document.add(Field::text(
        "contents".to_string(),
        contents,
    ));

    document.add(Field::text(
        "size".to_string(),
        format!("{}", file.metadata().unwrap().len()),
    ));

    document
}
