use crate::store::{Directory, InputStream};
use radix_fmt::radix_36;

#[derive(Debug)]
pub struct SegmentInfos {}

pub const SEGMENTS: &str = "segments";
pub const MAX_RADIX: u32 = 36;
pub const CODEC_MAGIC: u32 = 0x3fd76c17;
pub const ID_LENGTH: u32 = 16;

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

    let magic = input.read_u32();
    println!("magic - {}", magic);

    let codec = input.read_string();
    println!("codec - {}", codec);

    let version = input.read_int();
    println!("version - {}", version);

    let id = input.read_bytes(ID_LENGTH as usize);
    println!("id - {:?}", id);

    // Suffix should be generation
    let suffix_length = input.read_byte();
    println!("suffix_length - {}", suffix_length);

    let suffix_bytes = input.read_bytes(suffix_length as usize);
    println!("suffix_bytes - {:?}", suffix_bytes);

    let suffix = String::from_utf8(suffix_bytes).unwrap();
    println!("suffix - {}, generation - {}", suffix, radix_36(generation));

    let lucene_version = (input.read_vint(), input.read_vint(), input.read_vint());
    println!("lucene_version - {:?}", lucene_version);

    let index_created_version_major = input.read_vint();
    println!("index_created_version_major - {}", index_created_version_major);

    SegmentInfos {}
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
