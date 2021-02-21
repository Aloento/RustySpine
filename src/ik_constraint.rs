use crate::bone::Bone;
use crate::ik_constraint_data::IkConstraintData;
use crate::updatable::Updatable;
use crate::skeleton::Skeleton;

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
    pub fn new(data: &'a IkConstraintData<'a>, skeleton: &'a Skeleton<'a>) -> Self {
        let mut i = Self {
            bones: Vec::with_capacity(data.bones.len()),
            target: skeleton.findBone(data.target.unwrap().name),
            bendDirection: data.bendDirection,
            compress: data.compress,
            stretch: data.stretch,
            mix: data.mix,
            softness: data.softness,
            active: false
            data,
        }

        for boneData in i.data.bones {
            i.bones.push(skeleton.findBone(boneData.name));
        }

        return i;
    }
}
