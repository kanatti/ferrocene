use std::collections::{HashMap, HashSet};

use crate::store::{Directory, InputStream};

use super::{
    codec_utils,
    segment_info::{self, SegmentInfo},
};

/// Represents metadata about a specific segment commit
#[derive(Debug)]
pub struct SegmentCommitInfo {
    /// The segment info containing core segment metadata
    pub info: SegmentInfo,
    /// Deletion generation number for tracking deleted documents
    pub del_gen: i64,
    /// Number of deleted documents in this segment
    pub del_count: u32,
    /// Field infos generation number
    pub field_infos_gen: i64,
    /// Doc values generation number
    pub dv_gen: i64,
    /// Number of soft-deleted documents in this segment
    pub soft_delete_count: u32,
    /// Set of files containing field information
    pub field_infos_files: HashSet<String>,
    /// Map of field names to their doc values files
    pub dv_files: HashMap<String, String>,
}

pub fn read<I, D>(input: &mut I, directory: &D) -> SegmentCommitInfo
where
    I: InputStream,
    D: Directory,
{
    let segment_name = input.read_string();
    let segment_id = codec_utils::read_id(input);
    let _codec = input.read_string();

    let segment_info = segment_info::read(directory, &segment_name, &segment_id);

    let del_gen = input.read_long() as i64;
    let del_count = input.read_int();
    let field_infos_gen = input.read_long() as i64;
    let dv_gen = input.read_long() as i64;
    let soft_delete_count = input.read_int();
    let field_infos_files = input.read_set();

    let num_dv_fields = input.read_int();
    let mut dv_files = HashMap::new();

    // Read docvalues field names and their files
    for _ in 0..num_dv_fields {
        let field_name = input.read_string();
        let file_name = input.read_string();
        dv_files.insert(field_name, file_name);
    }

    SegmentCommitInfo {
        info: segment_info,
        del_gen,
        del_count,
        field_infos_gen,
        dv_gen,
        soft_delete_count,
        field_infos_files,
        dv_files,
    }
}
