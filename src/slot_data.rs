use crate::blend_mode::BlendMode;
use crate::bone_data::BoneData;
use crate::utils::color::Color;

pub struct SlotData<'b> {
    index: i32,
    name: String,
    boneData: &'b BoneData<'b>,
    color: Color,
    darkColor: Color,
    attachmentName: String,
    blendMode: BlendMode,
}

impl<'b> SlotData<'b> {
    pub fn new(index: i32, name: String, boneData: &'b BoneData<'b>) -> Self {
        SlotData {
            index,
            name,
            boneData,
            color: Color{
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: 1.0,
            },
            darkColor: Default::default(),
            attachmentName: "".to_string(),
            blendMode: BlendMode::Normal,
        }
    }
}
