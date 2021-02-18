use std::cell::Ref;
use std::fmt::Write;
use std::vec::IntoIter;

pub struct SkeletonInput {
    strings: Vec<String>,
    chars: Vec<char>,
    buffer: IntoIter<u8>,
}

impl SkeletonInput {
    pub fn new(file: String) -> Self {
        SkeletonInput {
            strings: vec![],
            chars: Vec::with_capacity(32),
            buffer: std::fs::read(file).unwrap().into_iter(),
        }
    }

    fn read_byte(&mut self) -> i32 {
        return self.buffer.next().unwrap() as i32;
    }

    pub fn read_int(&mut self, optimize: bool) -> i32 {
        let mut b = self.read_byte();
        let mut result = b & 0x7F;
        if (b & 0x80) != 0 {
            b = self.read_byte();
            result |= (b & 0x7F) << 7;
            if (b & 0x80) != 0 {
                b = self.read_byte();
                result |= (b & 0x7F) << 14;
                if (b & 0x80) != 0 {
                    b = self.read_byte();
                    result |= (b & 0x7F) << 21;
                    if (b & 0x80) != 0 {
                        b = self.read_byte();
                        result |= (b & 0x7F) << 28;
                    }
                }
            }
        };
        return if optimize {
            result
        } else {
            (result as u32 >> 1) as i32 ^ -(result & 1)
        };
    }

    pub fn read_native_int(&mut self) -> i32 {
        let ch1 = self.read_byte();
        let ch2 = self.read_byte();
        let ch3 = self.read_byte();
        let ch4 = self.read_byte();
        return (ch1 << 24) + (ch2 << 16) + (ch3 << 8) + (ch4 << 0);
    }

    pub fn read_float(&mut self) -> f32 {
        let i = self.read_native_int();
        f64::from(i) as f32
    }

    pub fn read_string_ref(&mut self) -> Option<&String> {
        let index = self.read_int(true) as usize;
        return if index == 0 {
            None
        } else {
            Some(self.strings.get(index - 1).unwrap())
        };
    }

    pub fn read_string(&mut self) -> Option<String> {
        let mut byte_count = self.read_int(true) as usize;
        if byte_count == 0 {
            return None;
        } else if byte_count == 1 {
            return Some("".to_string());
        }
        byte_count -= 1;

        if self.chars.len() < byte_count {
            self.chars = Vec::with_capacity(byte_count)
        }

        let mut char_count = 0;
        for mut i in 0..char_count {
            let mut b = self.read_byte();
            match b >> 4 {
                12 | 13 => {
                    self.chars[char_count] =
                        char::from_u32((((b & 0x1F) << 6 | self.read_byte() & 0x3F) as u32))
                            .unwrap();
                    i += 2;
                }
                14 => {
                    self.chars[char_count] = char::from_u32(
                        ((b & 0x0F) << 12
                            | (self.read_byte() & 0x3F) << 6
                            | self.read_byte() & 0x3F) as u32,
                    )
                        .unwrap();
                    i += 3;
                }
                _ => {
                    self.chars[char_count] = char::from_u32(b as u32).unwrap();
                    i += 1;
                }
            }
            char_count += 1;
        }

        let mut str = String::new();
        for char in &self.chars {
            str.write_char(*char).unwrap()
        }
        return Some(str);
    }
}
