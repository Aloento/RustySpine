use crate::blend_mode::BlendMode;
use crate::bone_data::BoneData;
use crate::utils::color::Color;

pub struct SlotData<'a> {
    index: i32,
    pub(crate) name: String,
    pub(crate) boneData: &'a BoneData<'a>,
    color: Color,
    pub(crate) darkColor: Option<Color>,
    attachmentName: String,
    blendMode: BlendMode,
}

impl<'a> SlotData<'a> {
    pub fn new(index: i32, name: String, boneData: &'a BoneData<'a>) -> Self {
        SlotData {
            index,
            name,
            boneData,
            color: Color {
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: 1.0,
            },
            darkColor: None,
            attachmentName: "".to_string(),
            blendMode: BlendMode::Normal,
        }
    }
}
