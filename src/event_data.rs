pub struct EvenData {
    name: String,
    intValue: i32,
    floatValue: f32,
    stringValue: String,
    audioPath: String,
    volume: f32,
    balance: f32,
}

impl EvenData {
    pub fn new(name: String) -> Self {
        EvenData {
            name,
            intValue: 0,
            floatValue: 0.0,
            stringValue: "".to_string(),
            audioPath: "".to_string(),
            volume: 0.0,
            balance: 0.0,
        }
    }
}
