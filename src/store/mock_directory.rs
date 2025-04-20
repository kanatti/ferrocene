use crate::store::{Directory, DirectoryError, InputStream, OutputStream};
use mockall::{mock, predicate::*};
use std::time::SystemTime;

mock! {
    pub InputStream {}

    impl InputStream for InputStream {
        fn read_exact(&mut self, buf: &mut [u8]);
    }
}

mock! {
    pub OutputStream {}

    impl OutputStream for OutputStream {
        fn write_byte(&mut self, value: u8);
        fn seek(&mut self, position: u64);
        fn stream_position(&mut self) -> u64;
        fn flush(&mut self);
    }
}

mock! {
    pub Directory {}

    impl Directory for Directory {
        type Output = MockOutputStream;
        type Input = MockInputStream;

        fn list(&self) -> Result<Vec<String>, DirectoryError>;
        fn file_exists(&self, name: &str) -> bool;
        fn file_modified_at(&self, name: &str) -> Result<SystemTime, DirectoryError>;
        fn file_length(&self, name: &str) -> Result<u64, DirectoryError>;
        fn delete_file(&self, name: &str) -> Result<(), DirectoryError>;
        fn rename_file(&self, from: &str, to: &str) -> Result<(), DirectoryError>;
        fn create_file(&self, name: &str) -> Result<MockOutputStream, DirectoryError>;
        fn open_file(&self, name: &str) -> Result<MockInputStream, DirectoryError>;
        fn close(&self) -> Result<(), DirectoryError>;
    }
}
