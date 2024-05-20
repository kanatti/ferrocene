pub const BUFFER_SIZE: usize = 1024;

/// Interface to read from a file in `Directory`.
/// Available when opening an existing file,
/// using `Directory::open_file`.
pub trait InputStream {
    fn read_byte(&mut self) -> u8;
    fn read_next(&mut self) -> char;
    fn unread_next(&mut self);
    fn get_next_char(&mut self) -> char;
    fn get_next_token(&mut self) -> String;
    fn get_next_int(&mut self) -> i32;
}

/// Interface to write to a file in `Directory`.
/// Since directory files are immutable,
/// this is available only from a newly created file,
/// using `Directory::create_file`.
pub trait OutputStream {
    fn write_byte(&mut self, value: u8);
    fn seek(&mut self, position: u64);
    fn stream_position(&mut self) -> u64;
    fn flush(&mut self);

    fn write_bytes(&mut self, values: &[u8]) {
        for value in values {
            self.write_byte(*value);
        }
    }

    fn write_u32(&mut self, value: u32) {
        self.write_byte((value >> 24) as u8);
        self.write_byte((value >> 16) as u8);
        self.write_byte((value >> 8) as u8);
        self.write_byte(value as u8);
    }

    fn write_int(&mut self, value: u32) {
        self.write_u32(value);
    }

    fn write_u64(&mut self, value: u64) {
        self.write_u32((value >> 32) as u32);
        self.write_u32(value as u32);
    }

    fn write_long(&mut self, value: u64) {
        self.write_u64(value);
    }

    fn write_vint(&mut self, value: u32) {
        let mut val = value;

        while val & !(0x7F) != 0 {
            self.write_byte(((val & 0x7f) | 0x80) as u8);
            val >>= 7;
        }

        self.write_byte(val as u8);
    }

    fn write_vlong(&mut self, value: u64) {
        let mut val = value;

        while val & !(0x7F) != 0 {
            self.write_byte(((val & 0x7f) | 0x80) as u8);
            val >>= 7;
        }

        self.write_byte(val as u8);
    }

    fn write_string(&mut self, value: &str) {
        self.write_vint(value.len() as u32);
        self.write_bytes(value.as_bytes());
    }
}

