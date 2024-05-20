use itertools::Itertools;
use std::{
    fs::{self, DirEntry, File},
    io::{BufReader, BufWriter, Seek, Write},
    path::PathBuf,
    time::SystemTime,
};

use super::{Directory, DirectoryError, InputStream, OutputStream};

pub struct FSDirectory {
    pub path: PathBuf,
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
            true => Ok(FSDirectory { path }),
            false => Err(DirectoryError::PathError(format!(
                "Path exists and is not a dir - {}",
                path.display()
            ))),
        }
    }

    fn entries(&self) -> Result<impl Iterator<Item = DirEntry>, DirectoryError> {
        Ok(fs::read_dir(&self.path)?.filter_map(|entry| entry.ok()))
    }
}

impl Directory for FSDirectory {
    type Input = FSInputStream;
    type Output = FSOutputStream;

    fn list(&self) -> Result<Vec<String>, DirectoryError> {
        let file_names: Vec<String> = self
            .entries()?
            .map(|e| e.file_name().to_string_lossy().to_string())
            .sorted()
            .collect();

        Ok(file_names)
    }

    fn file_exists(&self, name: &str) -> bool {
        let path = self.path.join(name);
        path.exists() && path.is_file()
    }

    fn file_modified_at(&self, name: &str) -> Result<SystemTime, DirectoryError> {
        let path = self.path.join(name);
        Ok(fs::metadata(path)?.modified()?)
    }

    fn file_length(&self, name: &str) -> Result<u64, DirectoryError> {
        let path = self.path.join(name);
        Ok(fs::metadata(&path)?.len())
    }

    fn delete_file(&self, name: &str) -> Result<(), DirectoryError> {
        let path = self.path.join(name);
        Ok(fs::remove_file(path)?)
    }

    fn rename_file(&self, from: &str, to: &str) -> Result<(), DirectoryError> {
        let old = self.path.join(from);
        let new = self.path.join(to);

        Ok(fs::rename(old, new)?)
    }

    fn create_file(&self, name: &str) -> Result<Self::Output, DirectoryError> {
        let path = self.path.join(name);
        Ok(fs::File::create_new(path)?.into())
    }

    fn open_file(&self, name: &str) -> Result<Self::Input, DirectoryError> {
        let path = self.path.join(name);
        Ok(fs::File::open(path)?.into())
    }

    fn close(&self) -> Result<(), DirectoryError> {
        Ok(())
    }
}

pub struct FSInputStream {
    reader: BufReader<File>,
}

impl InputStream for FSInputStream {
    fn read_byte(&mut self) -> u8 {
        todo!()
    }

    fn read_next(&mut self) -> char {
        todo!()
    }

    fn unread_next(&mut self) {
        todo!()
    }

    fn get_next_char(&mut self) -> char {
        todo!()
    }

    fn get_next_token(&mut self) -> String {
        todo!()
    }

    fn get_next_int(&mut self) -> i32 {
        todo!()
    }
}

pub struct FSOutputStream {
    writer: BufWriter<File>,
}

// TODO: Improve error handling
impl OutputStream for FSOutputStream {
    fn write_byte(&mut self, value: u8) {
        self.writer.write_all(&[value]).unwrap();
    }

    fn seek(&mut self, position: u64) {
        self.writer.seek(std::io::SeekFrom::Start(position)).unwrap();
    }

    fn stream_position(&mut self) -> u64 {
        self.writer.stream_position().unwrap()
    }

    fn flush(&mut self) {
        self.writer.flush().unwrap();
    }
}

impl Into<FSInputStream> for File {
    fn into(self) -> FSInputStream {
        FSInputStream {
            reader: BufReader::new(self),
        }
    }
}

impl Into<FSOutputStream> for File {
    fn into(self) -> FSOutputStream {
        FSOutputStream {
            writer: BufWriter::new(self),
        }
    }
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
