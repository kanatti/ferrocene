use std::{io, time::SystemTime};

use thiserror::Error;

use super::{InputStream, OutputStream};

/// Directory provides an abstraction layer for storing a list of files.
/// A directory contains only flat list of files, no sub-folder hierarchy.
/// A file can be created once, after which it can be only open for reading or deleting.
pub trait Directory {
    type Output: OutputStream;
    type Input: InputStream;

    /// Returns a list of files in the directory.
    fn list(&self) -> Result<Vec<String>, DirectoryError>;

    /// Returns true if the file exists.
    fn file_exists(&self, name: &str) -> bool;

    /// Returns the last modified time of the file.
    fn file_modified_at(&self, name: &str) -> Result<SystemTime, DirectoryError>;

    /// Returns the length of the file.
    fn file_length(&self, name: &str) -> Result<u64, DirectoryError>;

    /// Deletes the file.
    fn delete_file(&self, name: &str) -> Result<(), DirectoryError>;

    /// Renames the file.
    fn rename_file(&self, from: &str, to: &str) -> Result<(), DirectoryError>;

    /// Creates an empty file to write.
    fn create_file(&self, name: &str) -> Result<Self::Output, DirectoryError>;

    /// Open a file for reading.
    fn open_file(&self, name: &str) -> Result<Self::Input, DirectoryError>;

    // Close the store
    fn close(&self) -> Result<(), DirectoryError>;
}

/// Error type for Directory operations.
#[derive(Error, Debug)]
pub enum DirectoryError {
    #[error("IO Error")]
    IOError(#[from] io::Error),
    #[error("Path Error")]
    PathError(String),
    #[error("File Deleted Error")]
    FileDeletedError(String),
}
