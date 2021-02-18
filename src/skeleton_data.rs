use crate::bone_data::BoneData;
use crate::slot_data::SlotData;
use crate::skin::Skin;
use crate::event_data::EvenData;
use crate::animation::Animation;
use crate::ik_constraint_data::IkConstraintData;
use crate::transform_constraint_data::TransformConstraintData;
use crate::path_constraint_data::PathConstraintData;

pub struct SkeletonData<'b> {
    bones: Vec<BoneData<'b>>,
    slots: Vec<SlotData<'b>>,
    skins: Vec<Skin>,
    events: Vec<EvenData>,
    animations: Vec<Animation>,
    ikConstraints: Vec<IkConstraintData<'b>>,
    transformConstraints: Vec<TransformConstraintData<'b>>,
    pathConstraints: Vec<PathConstraintData<'b>>,
    name: String,
    defaultSkin: Skin,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    fps: f32,
    version: String,
    hash: String,
    imagesPath: String,
    audioPath: String,
}

impl<'b> SkeletonData<'b> {
    pub fn new() -> Self {
        SkeletonData {
            bones: vec![],
            slots: vec![],
            skins: vec![],
            events: vec![],
            animations: vec![],
            ikConstraints: vec![],
            transformConstraints: vec![],
            pathConstraints: vec![],
            name: "".to_string(),
            defaultSkin: Skin {},
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
            fps: 60.0,
            version: "".to_string(),
            hash: "".to_string(),
            imagesPath: "".to_string(),
            audioPath: "".to_string()
        }
    }
}
