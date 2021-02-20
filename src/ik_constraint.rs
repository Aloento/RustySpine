use crate::bone::Bone;
use crate::ik_constraint_data::IkConstraintData;
use crate::updatable::Updatable;

pub struct IkConstraint<'a> {
    data: &'a IkConstraintData<'a>,
    bones: Vec<Option<&'a Bone<'a>>>,
    target: Option<&'a Bone<'a>>,
    bendDirection: i32,
    compress: bool,
    stretch: bool,
    mix: f32,
    softness: f32,
    active: bool,
}

impl<'a> Updatable for IkConstraint<'a> {
    fn update(&self) {
        unimplemented!()
    }
}

impl<'a> IkConstraint<'a> {

}
