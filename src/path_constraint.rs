use crate::updatable::Updatable;
use crate::path_constraint_data::PathConstraintData;
use crate::bone::Bone;
use crate::slot::Slot;

const NONE: i8 = -1;
const BEFORE: i8 = -2;
const AFTER: i8 = -3;
const EPSILON: f32 = 0.00001;

pub struct PathConstraint<'a> {
    data: &'a PathConstraintData<'a>,
    bones: Vec<Option<&'a Bone<'a>>>,
    spaces: Vec<f32>,
    positions: Vec<f32>,
    world: Vec<f32>,
    curves: Vec<f32>,
    lengths: Vec<f32>,
    segments: Vec<f32>,
    target: Option<&'a Slot<'a>>,
    position: f32,
    spacing: f32,
    rotateMix: f32,
    translateMix: f32,
    active: bool,
}

impl<'a> Updatable for PathConstraint<'a> {
    fn update(&self) {
        unimplemented!()
    }
}

impl<'a> PathConstraint<'a> {}
