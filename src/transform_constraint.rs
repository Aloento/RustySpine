use crate::bone::Bone;
use crate::transform_constraint_data::TransformConstraintData;
use crate::updatable::Updatable;

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

impl<'a> TransformConstraint<'a> {}
