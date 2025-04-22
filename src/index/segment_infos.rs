use crate::{
    index::{codec_utils, segment_info},
    store::{Directory, InputStream},
};
use radix_fmt::radix_36;

#[derive(Debug)]
pub struct SegmentInfos {}

#[derive(Debug)]
pub struct SegmentCommitInfo {}

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

    let magic = input.read_u32();
    println!("magic - {}", magic);

    let codec = input.read_string();
    println!("codec - {}", codec);

    let format = input.read_int();
    println!("format - {}", format);

    let id = codec_utils::read_id(&mut input);
    println!("id - {:?}", id);

    // Suffix should be generation
    let suffix = codec_utils::read_suffix(&mut input);
    println!("suffix - {}, generation - {}", suffix, radix_36(generation));

    let lucene_version = (input.read_vint(), input.read_vint(), input.read_vint());
    println!("lucene_version - {:?}", lucene_version);

    let index_created_version_major = input.read_vint();
    println!(
        "index_created_version_major - {}",
        index_created_version_major
    );

    let sis_version = input.read_long();
    println!("sis_version - {}", sis_version);

    let sis_counter = input.read_vlong();
    println!("sis_counter - {}", sis_counter);

    let num_segments = input.read_int();
    println!("num_segments - {}", num_segments);

    if num_segments > 0 {
        let min_segment_lucene_version = (input.read_vint(), input.read_vint(), input.read_vint());
        println!(
            "min_segment_lucene_version - {:?}",
            min_segment_lucene_version
        );
    }

    // Read each segment-commit-info
    for _seg in 0..num_segments {
        let segment_name = input.read_string();
        println!("segment_name - {}", segment_name);

        let segment_id = codec_utils::read_id(&mut input);
        println!("segment_id - {:?}", segment_id);

        let codec = input.read_string();
        println!("codec - {}", codec);

        let segment_info = segment_info::read(directory, &segment_name, &segment_id);
        println!("SegmentInfo - {:#?}", segment_info);

        let del_gen = input.read_long();
        println!("del_gen - {}", del_gen as i64);

        let del_count = input.read_int();
        println!("del_count - {}", del_count);

        let field_infos_gen = input.read_long();
        println!("field_infos_gen - {}", field_infos_gen as i64);

        let dv_gen = input.read_long();
        println!("dv_gen - {}", dv_gen as i64);

        let soft_delete_count = input.read_int();
        println!("soft_delete_count - {}", soft_delete_count);

        let field_infos_files = input.read_set();
        println!("field_infos_files - {:?}", field_infos_files);

        let num_dv_fields = input.read_int();
        println!("num_dv_fields - {}", num_dv_fields);

        // TODO: Read dv file names if non-zero
    }

    // Read user data
    let user_data = input.read_map();
    println!("user_data - {:?}", user_data);

    // Footer
    let footer_magic = input.read_int();
    println!("footer_magic - {}", footer_magic as i32);

    let algorithm_id = input.read_int();
    println!("algorithm_id - {}", algorithm_id);

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
