use crate::event_data::EvenData;

pub struct Event<'a> {
    time: f32,
    data: &'a EvenData,
    intValue: i32,
    floatValue: f32,
    stringValue: String,
    volume: f32,
    balance: f32,
}

impl<'a> Event<'a> {
    pub fn new(time: f32, data: &'a EvenData) -> Self {
        Self {
            time,
            data,
            intValue: 0,
            floatValue: 0.0,
            stringValue: "".to_string(),
            volume: 0.0,
            balance: 0.0,
        }
    }
}
