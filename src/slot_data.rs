use crate::bone_data::BoneData;

pub struct SlotData {
    index: i32,
    name: String,
    boneData: *const BoneData,

}
