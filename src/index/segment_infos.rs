use std::collections::HashMap;

use crate::{
    index::{codec_utils, segment_commit_info::SegmentCommitInfo},
    store::{Directory, InputStream},
    version::Version,
};
use radix_fmt::radix_36;

use super::{codec_utils::Id, segment_commit_info};

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
    pub id: Id,
    /// Minimum Lucene version across all segments in the index
    pub min_segment_lucene_version: Option<Version>,
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
    let version = read_version(&mut input);

    let index_created_version_major = input.read_vint();
    let sis_version = input.read_long();
    let sis_counter = input.read_vlong();
    let num_segments = input.read_int();

    // Read minimum segment Lucene version if there are segments
    let min_segment_lucene_version = if num_segments > 0 {
        Some(read_version(&mut input))
    } else {
        None
    };

    // Read each segment-commit-info
    let mut segments = Vec::with_capacity(num_segments as usize);
    for _ in 0..num_segments {
        segments.push(segment_commit_info::read(&mut input, directory));
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

fn read_version<I: InputStream>(input: &mut I) -> Version {
    Version {
        major: input.read_vint(),
        minor: input.read_vint(),
        bugfix: input.read_vint(),
    }
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
