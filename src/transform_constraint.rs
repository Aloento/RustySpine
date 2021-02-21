use crate::bone::Bone;
use crate::transform_constraint_data::TransformConstraintData;
use crate::updatable::Updatable;
use crate::skeleton::Skeleton;

pub struct TransformConstraint<'a> {
    data: &'a TransformConstraintData<'a>,
    bones: Vec<Option<&'a Bone<'a>>>,
    target: Option<&'a Bone<'a>>,
    rotateMix: f32,
    translateMix: f32,
    scaleMix: f32,
    shearMix: f32,
    active: bool,
}

impl<'a> Updatable for TransformConstraint<'a> {
    fn update(&self) {
        unimplemented!()
    }
}

impl<'a> TransformConstraint<'a> {
    pub fn new(data: &'a TransformConstraintData<'a>, skeleton: &'a Skeleton<'a>) -> Self {
        let mut i = Self {
            bones: Vec::with_capacity(data.bones.len()),
            target: Some(skeleton.findBone(&data.target.unwrap().name)),
            rotateMix: data.rotateMix,
            translateMix: data.translateMix,
            scaleMix: data.scaleMix,
            shearMix: data.shearMix,
            active: false,
            data,
        };

        for boneData in i.data.bones {
            i.bones.push(skeleton.findBone(&boneData.name))
        }

        return i;
    }
}
