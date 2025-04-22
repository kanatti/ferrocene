use crate::store::InputStream;

pub const ID_LENGTH: u32 = 16;
pub const CODEC_MAGIC: u32 = 0x3fd76c17;


pub fn read_suffix<I: InputStream>(input: &mut I) -> String {
    let suffix_length = input.read_byte();
    println!("suffix_length - {}", suffix_length);

    let suffix_bytes = input.read_bytes(suffix_length as usize);
    println!("suffix_bytes - {:?}", suffix_bytes);

    return String::from_utf8(suffix_bytes).unwrap();
}

pub fn read_id<I: InputStream>(input: &mut I) -> Vec<u8> {
    input.read_bytes(ID_LENGTH as usize)
}

pub fn check_header<I: InputStream>(input: &mut I) {
    let magic = input.read_u32();
    assert!(magic == CODEC_MAGIC, "Magic not matching");
    // Check header to get version
    let codec = input.read_string();
    println!("codec - {}", codec);

    let version = input.read_int();
    println!("version - {}", version);
    // Check header ID
    let segment_id = input.read_bytes(ID_LENGTH as usize);
    println!("segment_id - {:?}", segment_id);

    // Check header suffix
    let suffix = read_suffix(input);
    println!("suffix - {:?}", suffix);
}

pub fn check_footer() {}
