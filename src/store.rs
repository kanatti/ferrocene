//! Directory provides an abstraction layer for storing a list of files.
//! A directory conatins only files, no sub-folder hierarchy.
//! Check org.apache.lucene.store.Directory for more details.

use itertools::Itertools;
use std::{collections::HashSet, fs, io, path::PathBuf};
use thiserror::Error;

pub trait Directory {
    fn list_all(&self) -> Result<Vec<String>, DirectoryError>;
    fn file_length(&self, name: &str) -> Result<u64, DirectoryError>;
}

pub struct FSDirectory {
    pub path: PathBuf,
    pending_deletes: HashSet<String>,
}

impl FSDirectory {
    pub fn new<P>(path: P) -> Result<FSDirectory, DirectoryError>
    where
        P: Into<PathBuf>,
    {
        let path = path.into();

        if !path.exists() {
            fs::create_dir(&path)?;
        }

        match path.is_dir() {
            true => Ok(FSDirectory {
                path,
                pending_deletes: HashSet::new(),
            }),
            false => Err(DirectoryError::PathError(format!(
                "Path exists and is not a dir - {}",
                path.display()
            ))),
        }
    }
}

impl Directory for FSDirectory {
    fn list_all(&self) -> Result<Vec<String>, DirectoryError> {
        let file_names: Vec<String> = fs::read_dir(&self.path)?
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let file_name = entry.file_name().to_string_lossy().to_string();

                match !self.pending_deletes.contains(&file_name) {
                    true => Some(file_name),
                    false => None,
                }
            })
            .sorted()
            .collect();

        Ok(file_names)
    }

    fn file_length(&self, name: &str) -> Result<u64, DirectoryError> {
        if self.pending_deletes.contains(name) {
            return Err(DirectoryError::FileDeletedError(format!(
                "File {} is pending delete",
                name.to_string()
            )));
        }

        let path = self.path.join(name);
        let metadata = fs::metadata(&path)?;

        Ok(metadata.len())
    }
}

#[derive(Error, Debug)]
pub enum DirectoryError {
    #[error("IO Error")]
    IOError(#[from] io::Error),
    #[error("Path Error")]
    PathError(String),
    #[error("File Deleted Error")]
    FileDeletedError(String),
}

#[cfg(test)]
mod tests {
    use std::fs::DirEntry;

    use super::*;

    macro_rules! assert_directory_contains_single_entry {
        ($dir:expr, $entry:expr) => {{
            // List dir
            let entries: Vec<DirEntry> = fs::read_dir($dir)
                .expect("Failed to read temp dir")
                .map(|entry| entry.expect("Failed to read dir entry"))
                .collect();

            assert_eq!(
                entries.len(),
                1,
                "Unexpected number of entries in directory"
            );
            assert_eq!(
                entries[0].path(),
                $dir.join($entry),
                "Unexpected entry path"
            );
        }};
    }

    #[test]
    fn new_when_dir_absent() {
        // Setup a temp-dir that will be cleaned up after test
        let root_dir = tempfile::tempdir().expect("Failed to create temp dir");

        let path = root_dir.path().join("test-index");

        let directory = FSDirectory::new(path);

        assert!(directory.is_ok());
        assert_directory_contains_single_entry!(root_dir.path(), "test-index");

        root_dir.close().expect("Failed to close temp dir");
    }

    #[test]
    fn new_when_dir_exists() {
        // Setup a temp-dir that will be cleaned up after test
        let root_dir = tempfile::tempdir().expect("Failed to create temp dir");

        let path = root_dir.path().join("test-index");
        let file_path = path.join("test-file.txt");

        // Create the backing dir and file
        fs::create_dir(&path).expect("Failed to create directory");
        fs::File::create(file_path).expect("Failed to create file");

        let directory = FSDirectory::new(path.clone());

        assert!(directory.is_ok());
        assert_directory_contains_single_entry!(root_dir.path(), "test-index");
        assert_directory_contains_single_entry!(&path, "test-file.txt");

        root_dir.close().expect("Failed to close temp dir");
    }

    #[test]
    fn new_when_path_is_file() {
        // Setup a temp-dir that will be cleaned up after test
        let root_dir = tempfile::tempdir().expect("Failed to create temp dir");

        let path = root_dir.path().join("test-index");

        // Create a file insteead of dir
        fs::File::create(&path).expect("Failed to create file");

        let directory = FSDirectory::new(path);

        assert!(matches!(
            directory.err().expect("Expected error"),
            DirectoryError::PathError(_)
        ));

        root_dir.close().expect("Failed to close temp dir");
    }
}
