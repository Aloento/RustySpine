use crate::bone_data::BoneData;

pub struct SkeletonData<'b> {
    bones: Vec<BoneData<'b>>,
    // slots: Vec<>,
}
