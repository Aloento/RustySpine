use crate::attachments::attachment::Attachment;
use crate::attachments::texture_region::TextureRegion;
use crate::utils::color::Color;

const BLX: i32 = 0;
const BLY: i32 = 1;
const ULX: i32 = 2;
const ULY: i32 = 3;
const URX: i32 = 4;
const URY: i32 = 5;
const BRX: i32 = 6;
const BRY: i32 = 7;

pub struct RegionAttachment {
    Attachment: Attachment,
    uvs: Vec<f32>,
    offset: Vec<f32>,
    color: Color,
    region: TextureRegion,
    path: String,
    x: f32,
    y: f32,
    scaleX: f32,
    scaleY: f32,
    rotation: f32,
    width: f32,
    height: f32,
}

impl RegionAttachment {
    pub fn new(name: String) -> Self {
        Self {
            Attachment: Attachment::new(name),
            uvs: Vec::with_capacity(8),
            offset: Vec::with_capacity(8),
            color: Color::new(),
            region: TextureRegion {},
            path: "".to_string(),
            x: 0.0,
            y: 0.0,
            scaleX: 1.0,
            scaleY: 1.0,
            rotation: 0.0,
            width: 0.0,
            height: 0.0,
        }
    }
}
