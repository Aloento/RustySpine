pub struct SkeletonInput {
    buffer: Vec<u8>,
    strings: Vec<String>,
    chars: [char; 32]
}

impl SkeletonInput {
    pub fn new(file: String) -> SkeletonInput {
        SkeletonInput {
            buffer: std::fs::read(file).unwrap(),
            strings: vec![],
            chars: [char; 32]
        }
    }
}
