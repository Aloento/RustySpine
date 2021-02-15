pub struct SkeletonInput {
    strings: Vec<String>,
    chars: [char; 32],
    buffer: Vec<u8>,
    index: u32
}

impl SkeletonInput {
    pub fn new(file: String) -> SkeletonInput {
        SkeletonInput {
            strings: vec![],
            chars: [char; 32],
            buffer: std::fs::read(file).unwrap(),
            index: 0
        }
    }

    pub fn read_byte(&mut self) -> i32 {
        let i = self.buffer.get(self.index).unwrap() as i32;
        self.index = self.index + 1;
        return i
    }

    pub fn read_int() -> i32 {

    }

    pub fn read_string_ref() -> String {

    }
}
