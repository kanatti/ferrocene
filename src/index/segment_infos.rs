use crate::store::Directory;
use radix_fmt::radix_36;

pub struct SegmentInfos {}

pub const SEGMENTS: &str = "segments";
pub const MAX_RADIX: u32 = 36;

impl SegmentInfos {
    pub fn get_last_segments_file_name(directory: impl Directory) -> String {
        let files = directory.list().unwrap();
        let gen = SegmentInfos::get_last_commit_generation(files);
        let gen_base_36 = radix_36(gen);
        format!(
            "{}_{}",
            SEGMENTS,
            gen_base_36
        )
    }

    pub fn get_last_commit_generation(files: Vec<String>) -> u64 {
        files
            .iter()
            .filter(|&f| f.starts_with(SEGMENTS))
            .map(|f| SegmentInfos::get_generation_from_file_name(f))
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::store::mock_directory::MockDirectory;

    #[test]
    fn test_get_last_segments_file_name() {
        let mut mock_directory = MockDirectory::new();

        mock_directory
            .expect_list()
            .returning(|| Ok(vec![
                "segments_10".to_string(),
                "segments_1".to_string(),
                "segments_2".to_string(),
                "_1.si".to_string()
            ]));

        let result = SegmentInfos::get_last_segments_file_name(mock_directory);

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
            assert_eq!(SegmentInfos::get_last_commit_generation(files), expected);
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
            assert_eq!(
                SegmentInfos::get_generation_from_file_name(file_name),
                expected
            );
        }
    }
}
