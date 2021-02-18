use crate::bone_data::BoneData;
use crate::slot_data::SlotData;
use crate::skin::Skin;
use crate::event_data::EvenData;

pub struct SkeletonData<'b> {
    bones: Vec<BoneData<'b>>,
    slots: Vec<SlotData<'b>>,
    skins: Vec<Skin>,
    events: Vec<EvenData>,
    
}
