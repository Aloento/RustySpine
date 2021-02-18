use crate::bone_data::BoneData;
use crate::constraint_data::ConstraintData;

pub struct IkConstraintData<'b> {
    data: ConstraintData,
    bones: Vec<&'b BoneData<'b>>,
    target: Option<&'b BoneData<'b>>,
    bendDirection: i32,
    compress: bool,
    stretch: bool,
    uniform: bool,
    mix: f32,
    softness: f32,
}

impl<'b> IkConstraintData<'b> {
    pub fn new(name: String) -> Self {
        IkConstraintData {
            data: ConstraintData::new(name),
            bones: vec![],
            target: None,
            bendDirection: 1,
            compress: false,
            stretch: false,
            uniform: false,
            mix: 1.0,
            softness: 0.0,
        }
    }
}
