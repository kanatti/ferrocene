use std::collections::{HashMap, HashSet};

use crate::{
    index::{codec_utils, segment_info},
    store::{Directory, InputStream},
    version::Version,
};
use radix_fmt::radix_36;

use super::segment_info::SegmentInfo;

/// Represents metadata about all segments in the index
#[derive(Debug)]
pub struct SegmentInfos {
    /// Generation number of the segments file
    pub generation: u64,
    /// Lucene version used to write this segments file
    pub version: Version,
    /// Major version of Lucene when the index was created
    pub index_created_version_major: u32,
    /// Segment infos version number
    pub sis_version: u64,
    /// Counter used to track segment creation
    pub sis_counter: u64,
    /// List of segment commit infos for all segments in the index
    pub segments: Vec<SegmentCommitInfo>,
    /// User-defined metadata for the index
    pub user_data: HashMap<String, String>,
    /// Unique identifier for this segment infos
    pub id: Vec<u8>,
    /// Minimum Lucene version across all segments in the index
    pub min_segment_lucene_version: Option<Version>,
}

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

pub const SEGMENTS: &str = "segments";
pub const MAX_RADIX: u32 = 36;

pub const SEG_INFO_CODEC: &str = "Lucene70SegmentInfo";

pub fn get_last_segments_file_name<D: Directory>(directory: &D) -> String {
    let files = directory.list().unwrap();
    let gen = get_last_commit_generation(files);
    let gen_base_36 = radix_36(gen);
    format!("{}_{}", SEGMENTS, gen_base_36)
}

pub fn get_last_commit_generation(files: Vec<String>) -> u64 {
    files
        .iter()
        .filter(|&f| f.starts_with(SEGMENTS))
        .map(|f| get_generation_from_file_name(f))
        .max()
        .unwrap()
}

pub fn get_generation_from_file_name(file_name: impl AsRef<str>) -> u64 {
    let file_name = file_name.as_ref();

    if file_name == SEGMENTS {
        return 0;
    } else {
        let segment_length = SEGMENTS.len();
        let sub_str = &file_name[1 + segment_length..];

        u64::from_str_radix(sub_str, MAX_RADIX).unwrap()
    }
}

pub fn read_segment_infos<D: Directory>(
    directory: &D,
    segments_file: impl AsRef<str>,
) -> SegmentInfos {
    let segments_file = segments_file.as_ref();
    let generation: u64 = get_generation_from_file_name(segments_file);

    let mut input = directory.open_file(segments_file).unwrap();

    let _magic = input.read_u32();
    let _codec = input.read_string();
    let _format = input.read_int();

    let id = codec_utils::read_id(&mut input);
    let _suffix = codec_utils::read_suffix(&mut input);

    // Read Lucene version
    let version = Version {
        major: input.read_vint(),
        minor: input.read_vint(),
        bugfix: input.read_vint(),
    };

    let index_created_version_major = input.read_vint();
    let sis_version = input.read_long();
    let sis_counter = input.read_vlong();
    let num_segments = input.read_int();

    // Read minimum segment Lucene version if there are segments
    let min_segment_lucene_version = if num_segments > 0 {
        Some(Version {
            major: input.read_vint(),
            minor: input.read_vint(),
            bugfix: input.read_vint(),
        })
    } else {
        None
    };

    // Read each segment-commit-info
    let mut segments = Vec::with_capacity(num_segments as usize);
    for _ in 0..num_segments {
        let segment_name = input.read_string();
        let segment_id = codec_utils::read_id(&mut input);
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

        segments.push(SegmentCommitInfo {
            info: segment_info,
            del_gen,
            del_count,
            field_infos_gen,
            dv_gen,
            soft_delete_count,
            field_infos_files,
            dv_files,
        });
    }

    // Read user data
    let user_data = input.read_map();

    // Read footer
    let _footer_magic = input.read_int();
    let _algorithm_id = input.read_int();

    SegmentInfos {
        generation,
        version,
        index_created_version_major,
        sis_version,
        sis_counter,
        segments,
        user_data,
        id,
        min_segment_lucene_version,
    }
}

pub fn read_latest_segment_infos<D: Directory>(directory: &D) -> SegmentInfos {
    let segments_file = get_last_segments_file_name(directory);
    read_segment_infos(directory, segments_file)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::store::mock_directory::MockDirectory;

    #[test]
    fn test_get_last_segments_file_name() {
        let mut mock_directory = MockDirectory::new();

        mock_directory.expect_list().returning(|| {
            Ok(vec![
                "segments_10".to_string(),
                "segments_1".to_string(),
                "segments_2".to_string(),
                "_1.si".to_string(),
            ])
        });

        let result = get_last_segments_file_name(&mock_directory);

        assert_eq!(result, "segments_10");
    }

    #[test]
    fn test_get_last_commit_generation() {
        let test_cases = vec![
            (vec!["segments".to_string()], 0),
            (vec!["segments_1".to_string()], 1),
            (vec!["segments_1".to_string(), "segments_2".to_string()], 2),
            (
                vec![
                    "segments_2".to_string(),
                    "segments_1".to_string(),
                    "_1.si".to_string(),
                ],
                2,
            ),
            (
                vec![
                    "segments_1".to_string(),
                    "segments_5".to_string(),
                    "segments_2".to_string(),
                ],
                5,
            ),
        ];

        for (files, expected) in test_cases {
            assert_eq!(get_last_commit_generation(files), expected);
        }
    }

    #[test]
    fn test_get_generation_from_file_name() {
        let test_cases = vec![
            ("segments", 0),        // No number after '_', expect 0
            ("segments_1", 1),      // "1" in base-36 = 1
            ("segments_10", 36),    // "10" in base-36 = 36
            ("segments_36", 114),   // "36" in base-36 = 3*36 + 6 = 114
            ("segments_100", 1296), // "100" in base-36 = 1*36^2 = 1296
        ];

        for (file_name, expected) in test_cases {
            assert_eq!(get_generation_from_file_name(file_name), expected);
        }
    }
}
