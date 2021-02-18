use crate::bone_data::BoneData;
use crate::slot_data::SlotData;
use crate::skin::Skin;
use crate::event_data::EvenData;
use crate::animation::Animation;
use crate::ik_constraint_data::IkConstraintData;
use crate::transform_constraint_data::TransformConstraintData;

pub struct SkeletonData<'b> {
    bones: Vec<BoneData<'b>>,
    slots: Vec<SlotData<'b>>,
    skins: Vec<Skin>,
    events: Vec<EvenData>,
    animations: Vec<Animation>,
    ikConstraints: Vec<IkConstraintData<'b>>,
    transformConstraints: Vec<TransformConstraintData<'b>>,

}
