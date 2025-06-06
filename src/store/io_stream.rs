use std::collections::{HashMap, HashSet};

pub const BUFFER_SIZE: usize = 1024;

/// Interface to read from a file in `Directory`.
/// Available when opening an existing file,
/// using `Directory::open_file`.
pub trait InputStream {
    fn read_exact(&mut self, buf: &mut [u8]);

    fn read_byte(&mut self) -> u8 {
        let mut buf = [0; 1];
        self.read_exact(&mut buf);
        buf[0]
    }

    fn read_bytes(&mut self, len: usize) -> Vec<u8> {
        let mut buf = vec![0; len];
        self.read_exact(&mut buf);
        buf
    }

    fn read_bool(&mut self) -> bool {
        self.read_byte() != 0
    }

    fn read_u32(&mut self) -> u32 {
        let mut buf = [0; 4];
        self.read_exact(&mut buf);
        u32::from_be_bytes(buf)
    }

    fn read_int(&mut self) -> u32 {
        self.read_u32()
    }

    fn read_u64(&mut self) -> u64 {
        let mut buf = [0; 8];
        self.read_exact(&mut buf);
        u64::from_be_bytes(buf)
    }

    fn read_long(&mut self) -> u64 {
        self.read_u64()
    }

    fn read_vint(&mut self) -> u32 {
        let mut value = 0;
        let mut shift = 0;

        loop {
            let b = self.read_byte();
            value |= ((b & 0x7F) as u32) << shift;
            shift += 7;

            if (b & 0x80) == 0 {
                break;
            }
        }

        value
    }

    fn read_vlong(&mut self) -> u64 {
        let mut value = 0;
        let mut shift = 0;

        loop {
            let b = self.read_byte();
            value |= ((b & 0x7F) as u64) << shift;
            shift += 7;

            if (b & 0x80) == 0 {
                break;
            }
        }

        value
    }

    fn read_string(&mut self) -> String {
        let len = self.read_vint();
        let mut buf = vec![0; len as usize];
        self.read_exact(&mut buf);
        String::from_utf8(buf).unwrap()
    }

    fn read_map(&mut self) -> HashMap<String, String> {
        let count = self.read_vint() as usize;
        let mut map = HashMap::with_capacity(count);

        for _ in 0..count {
            let key = self.read_string();
            let value = self.read_string();

            map.insert(key, value);
        }

        map
    }

    fn read_set(&mut self) -> HashSet<String> {
        let count = self.read_vint() as usize;
        let mut set = HashSet::with_capacity(count);

        for _ in 0..count {
            let value = self.read_string();
            set.insert(value);
        }

        set
    }

    fn read_vec(&mut self) -> Vec<String> {
        let count = self.read_vint() as usize;
        let mut vec = Vec::with_capacity(count);

        for _ in 0..count {
            let value = self.read_string();
            vec.push(value);
        }

        vec
    }
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

    fn write_bool(&mut self, value: bool) {
        self.write_byte(value as u8);
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
