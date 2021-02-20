use crate::animation::Animation;
use crate::bone_data::BoneData;
use crate::event_data::EvenData;
use crate::ik_constraint_data::IkConstraintData;
use crate::path_constraint_data::PathConstraintData;
use crate::skin::Skin;
use crate::slot_data::SlotData;
use crate::transform_constraint_data::TransformConstraintData;

pub struct SkeletonData<'a> {
    pub(crate) bones: Vec<BoneData<'a>>,
    pub(crate) slots: Vec<SlotData<'a>>,
    skins: Vec<Skin<'a>>,
    events: Vec<EvenData>,
    animations: Vec<Animation>,
    pub(crate) ikConstraints: Vec<IkConstraintData<'a>>,
    pub(crate) transformConstraints: Vec<TransformConstraintData<'a>>,
    pub(crate) pathConstraints: Vec<PathConstraintData<'a>>,
    name: String,
    defaultSkin: Option<Skin<'a>>,
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

impl<'a> SkeletonData<'a> {
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
            defaultSkin: None,
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
            fps: 60.0,
            version: "".to_string(),
            hash: "".to_string(),
            imagesPath: "".to_string(),
            audioPath: "".to_string(),
        }
    }

    pub fn find_bone(&self, name: String) -> Option<&BoneData> {
        if name.is_empty() {
            panic!("boneName cannot be null.")
        };
        for i in 0..self.bones.len() {
            let bone = self.bones.get(i).unwrap();
            if bone.name.eq(&name) {
                return Some(bone);
            }
        }
        return None;
    }

    pub fn find_slot(&self, name: String) -> Option<&SlotData> {
        if name.is_empty() {
            panic!("slotName cannot be null.")
        };
        for i in 0..self.slots.len() {
            let slot = self.slots.get(i).unwrap();
            if slot.name.eq(&name) {
                return Some(slot);
            }
        }
        return None;
    }

    pub fn find_skin(&self, name: String) -> Option<&Skin> {
        if name.is_empty() {
            panic!("skinName cannot be null.")
        };
        for i in &self.skins {
            if i.name.eq(&name) {
                return Some(i);
            }
        }
        return None;
    }
}
