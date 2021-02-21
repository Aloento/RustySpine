use std::borrow::Borrow;

use crate::bone::Bone;
use crate::ik_constraint::IkConstraint;
use crate::path_constraint::PathConstraint;
use crate::skeleton_data::SkeletonData;
use crate::skin::Skin;
use crate::slot::Slot;
use crate::transform_constraint::TransformConstraint;
use crate::updatable::Updatable;
use crate::utils::color::Color;

pub struct Skeleton<'a> {
    data: SkeletonData<'a>,
    bones: Vec<Bone<'a>>,
    pub(crate) slots: Vec<Slot<'a>>,
    ikConstraints: Vec<IkConstraint<'a>>,
    transformConstraints: Vec<TransformConstraint<'a>>,
    pathConstraints: Vec<PathConstraint<'a>>,
    updateCache: Vec<Box<dyn Updatable>>,
    updateCacheReset: Vec<&'a Bone<'a>>,
    color: Color,
    drawOrder: Vec<&'a Slot<'a>>,
    skin: Option<&'a Skin<'a>>,
    pub(crate) time: f32,
    scaleX: f32,
    scaleY: f32,
    x: f32,
    y: f32,
}

impl<'a> Skeleton<'a> {
    pub fn new(data: SkeletonData<'a>) -> Self {
        let mut i = Self {
            bones: Vec::with_capacity(data.bones.len()),
            slots: Vec::with_capacity(data.slots.len()),
            ikConstraints: Vec::with_capacity(data.ikConstraints.len()),
            transformConstraints: Vec::with_capacity(data.transformConstraints.len()),
            pathConstraints: Vec::with_capacity(data.pathConstraints.len()),
            updateCache: vec![],
            updateCacheReset: vec![],
            color: Color {
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: 1.0,
            },
            drawOrder: Vec::with_capacity(data.slots.len()),
            skin: None,
            time: 0.0,
            scaleX: 1.0,
            scaleY: 1.0,
            x: 0.0,
            y: 0.0,
            data,
        };

        for boneData in &i.data.bones {
            let bone;
            if boneData.parent.is_none() {
                bone = Bone::new(boneData, &i, None);
            } else {
                let mut parent = i.bones.get_mut(boneData.parent.unwrap().index as usize);
                parent.unwrap().children.push(&bone);
                bone = Bone::new(boneData, &i, parent.as_deref());
            }
            i.bones.push(bone);
        }

        for slotData in &i.data.slots {
            let bone = i.bones.get(slotData.boneData.index as usize);
            let slot = Slot::new(slotData, bone.unwrap());
            i.slots.push(slot);
            i.drawOrder.push(&slot);
        }

        for ikConstraintData in &i.data.ikConstraints {
            i.ikConstraints.push(IkConstraint::new(ikConstraintData, &i));
        }

        for transformConstraintData in &i.data.transformConstraints {
            i.transformConstraints.push(TransformConstraint::new(transformConstraintData, &i));
        }

        for pathConstraintData in &i.data.pathConstraints {
            i.pathConstraints.push(PathConstraint::new(pathConstraintData, &i));
        }

        return i;
    }

    pub fn findBone(boneName: &String) -> Option<&Bone> {
        None
    }

    pub fn findSlot(slotName: &String) -> Option<&Slot> {
        None
    }
}
