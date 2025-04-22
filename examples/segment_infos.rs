use ferrocene::{index::segment_infos, store::FSDirectory};

// Usage: cargo run --example segment_infos full-path-to-index
fn main() {
    let path = std::env::args().nth(1).expect("No path given");
    println!("Path: {}", path);
    let directory = FSDirectory::new(path).unwrap();
    let sis = segment_infos::read_latest_segment_infos(&directory);
    println!("Segment Infos: {:#?}", sis);
}


// Segment Infos: SegmentInfos {
//     generation: 4,
//     version: Version {
//         major: 7,
//         minor: 7,
//         bugfix: 3,
//     },
//     index_created_version_major: 7,
//     sis_version: 13,
//     sis_counter: 2,
//     segments: [
//         SegmentCommitInfo {
//             info: SegmentInfo {
//                 name: "_0",
//                 id: Id(1sp8xljstdpj25v681bybjo1m),
//                 version: Version {
//                     major: 7,
//                     minor: 7,
//                     bugfix: 3,
//                 },
//                 min_version: Some(
//                     Version {
//                         major: 7,
//                         minor: 7,
//                         bugfix: 3,
//                     },
//                 ),
//                 doc_count: 2,
//                 is_compound: true,
//                 diagnostics: {
//                     "lucene.version": "7.7.3",
//                     "java.runtime.version": "15.0.1+9",
//                     "source": "flush",
//                     "os": "Linux",
//                     "os.arch": "amd64",
//                     "java.vendor": "AdoptOpenJDK",
//                     "timestamp": "1741956578408",
//                     "java.version": "15.0.1",
//                     "os.version": "5.10.234-205.895.amzn2int.x86_64",
//                     "java.vm.version": "15.0.1+9",
//                 },
//                 files: {
//                     "_0.si",
//                     "_0.cfe",
//                     "_0.cfs",
//                 },
//                 attributes: {
//                     "Lucene50StoredFieldsFormat.mode": "BEST_SPEED",
//                 },
//                 num_sort_fields: 0,
//             },
//             del_gen: -1,
//             del_count: 0,
//             field_infos_gen: -1,
//             dv_gen: -1,
//             soft_delete_count: 0,
//             field_infos_files: {},
//             dv_files: {},
//         },
//         SegmentCommitInfo {
//             info: SegmentInfo {
//                 name: "_1",
//                 id: Id(1sp8xljstdpj25v681bybjo1p),
//                 version: Version {
//                     major: 7,
//                     minor: 7,
//                     bugfix: 3,
//                 },
//                 min_version: Some(
//                     Version {
//                         major: 7,
//                         minor: 7,
//                         bugfix: 3,
//                     },
//                 ),
//                 doc_count: 1,
//                 is_compound: true,
//                 diagnostics: {
//                     "java.runtime.version": "15.0.1+9",
//                     "java.vm.version": "15.0.1+9",
//                     "lucene.version": "7.7.3",
//                     "java.vendor": "AdoptOpenJDK",
//                     "os": "Linux",
//                     "java.version": "15.0.1",
//                     "os.version": "5.10.234-205.895.amzn2int.x86_64",
//                     "timestamp": "1741956730726",
//                     "os.arch": "amd64",
//                     "source": "flush",
//                 },
//                 files: {
//                     "_1.cfs",
//                     "_1.cfe",
//                     "_1.si",
//                 },
//                 attributes: {
//                     "Lucene50StoredFieldsFormat.mode": "BEST_SPEED",
//                 },
//                 num_sort_fields: 0,
//             },
//             del_gen: -1,
//             del_count: 0,
//             field_infos_gen: -1,
//             dv_gen: -1,
//             soft_delete_count: 0,
//             field_infos_files: {},
//             dv_files: {},
//         },
//     ],
//     user_data: {
//         "local_checkpoint": "2",
//         "translog_uuid": "AiS2-7CUQ3e9sQ4QOOyU6Q",
//         "history_uuid": "WR5G8EnsRzKHuV__MrC5gA",
//         "translog_generation": "3",
//         "sync_id": "1eV5NJKlRNujthlq48ZxiQ",
//         "max_seq_no": "2",
//         "max_unsafe_auto_id_timestamp": "-1",
//     },
//     id: Id(1sp8xljstdpj25v681bybjo1t),
//     min_segment_lucene_version: Some(
//         Version {
//             major: 7,
//             minor: 7,
//             bugfix: 3,
//         },
//     ),
// }