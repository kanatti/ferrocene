pub trait InputStream {
    fn read_byte(&mut self) -> u8;
    fn read_next(&mut self) -> char;
    fn unread_next(&mut self);
    fn get_next_char(&mut self) -> char;
    fn get_next_token(&mut self) -> String;
    fn get_next_int(&mut self) -> i32;
}

pub trait OutputStream {
    fn write_long(&mut self, value: u32);

    fn write_vint(&mut self, value: u64);

    fn write_byte(&mut self, value: u8);

    fn write_string(&mut self, value: &str);

    fn get_pointer(&self) -> u32;
}

