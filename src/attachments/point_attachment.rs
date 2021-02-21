use crate::attachments::attachment::Attachment;
use crate::utils::color::Color;

pub struct PointAttachment {
    Attachment: Attachment,
    color: Color,
    x: f32,
    y: f32,
    rotation: f32,
}

impl PointAttachment {
    pub fn new(name: String) -> Self {
        Self {
            Attachment: Attachment::new(name),
            color: Color {
                r: 0.9451,
                g: 0.9451,
                b: 0.0,
                a: 1.0,
            },
            x: 0.0,
            y: 0.0,
            rotation: 0.0,
        }
    }
}
