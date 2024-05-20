pub const BUFFER_SIZE: usize = 1024;

pub trait InputStream {
    fn read_byte(&mut self) -> u8;
    fn read_next(&mut self) -> char;
    fn unread_next(&mut self);
    fn get_next_char(&mut self) -> char;
    fn get_next_token(&mut self) -> String;
    fn get_next_int(&mut self) -> i32;
}

pub trait OutputStream {

}