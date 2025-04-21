use ferrocene::{index::segment_infos, store::FSDirectory};

// Usage: cargo run --example segment_infos full-path-to-index
fn main() {
    let path = std::env::args().nth(1).expect("No path given");
    println!("Path: {}", path);
    let directory = FSDirectory::new(path).unwrap();
    let sis = segment_infos::read_latest_segment_infos(&directory);
    println!("Segment Infos: {:?}", sis);
}
