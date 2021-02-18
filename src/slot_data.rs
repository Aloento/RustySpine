use crate::bone_data::BoneData;

pub struct SlotData<'b> {
    index: i32,
    name: String,
    boneData: &'b BoneData<'b>,
}
