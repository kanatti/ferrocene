use std::collections::{HashMap, HashSet};

use crate::{
    index::codec_utils,
    store::{Directory, InputStream},
    version::Version,
};

/// Represents metadata about a segment in the index
#[derive(Debug)]
pub struct SegmentInfo {
    /// Name of the segment
    pub name: String,
    /// Unique identifier for the segment
    pub id: Vec<u8>,
    /// Version of Lucene that created this segment
    pub version: Version,
    /// Minimum version of Lucene that can read this segment
    pub min_version: Option<Version>,
    /// Number of documents in this segment
    pub doc_count: u32,
    /// Whether this segment uses compound file format
    pub is_compound: bool,
    /// Diagnostic information about the segment
    pub diagnostics: HashMap<String, String>,
    /// Set of files that belong to this segment
    pub files: HashSet<String>,
    /// Additional attributes for this segment
    pub attributes: HashMap<String, String>,
    /// Number of sort fields in this segment
    pub num_sort_fields: u32,
}

pub const SEG_INFO_EXTENSION: &str = "si";

// SegmentInfo (si) reading, based on Lucene70 codec
// TODO: Make it codec specific.
pub fn read<D: Directory>(directory: &D, segment_name: &str, segment_id: &Vec<u8>) -> SegmentInfo {
    let si_file = format!("{}.{}", segment_name, SEG_INFO_EXTENSION);
    let mut input = directory.open_file(&si_file).unwrap();

    codec_utils::check_header(&mut input);

    // Read version information
    let version = Version {
        major: input.read_int(),
        minor: input.read_int(),
        bugfix: input.read_int(),
    };

    // Read minimum version if present
    let has_min_version = input.read_byte() != 0;
    let min_version = if has_min_version {
        Some(Version {
            major: input.read_int(),
            minor: input.read_int(),
            bugfix: input.read_int(),
        })
    } else {
        None
    };

    let doc_count = input.read_int();
    let is_compound = input.read_byte() != 0;

    let diagnostics = input.read_map();
    let files = input.read_set();
    let attributes = input.read_map();
    let num_sort_fields = input.read_vint();

    // TODO: Read IndexSort

    SegmentInfo {
        name: segment_name.to_string(),
        id: segment_id.clone(),
        version,
        min_version,
        doc_count,
        is_compound,
        diagnostics,
        files,
        attributes,
        num_sort_fields,
    }
}
