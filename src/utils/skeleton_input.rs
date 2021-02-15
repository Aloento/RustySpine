use std::vec::IntoIter;

pub struct SkeletonInput {
    strings: Vec<String>,
    chars: Vec<char>,
    buffer: IntoIter<u8>,
}

impl SkeletonInput {
    pub fn new(file: String) -> SkeletonInput {
        SkeletonInput {
            strings: vec![],
            chars: Vec::with_capacity(32),
            buffer: std::fs::read(file).unwrap().into_iter(),
        }
    }

    pub fn read_byte(&mut self) -> i32 {
        return self.buffer.next().unwrap() as i32;
    }

    pub fn read_int() -> i32 {}

    pub fn read_string_ref() -> String {}
}
