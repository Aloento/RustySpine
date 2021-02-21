use crate::bone_data::BoneData;
use crate::constraint_data::ConstraintData;

pub struct IkConstraintData<'a> {
    data: ConstraintData,
    pub(crate) bones: Vec<&'a BoneData<'a>>,
    pub(crate) target: Option<&'a BoneData<'a>>,
    pub(crate) bendDirection: i32,
    pub(crate) compress: bool,
    pub(crate) stretch: bool,
    uniform: bool,
    pub(crate) mix: f32,
    pub(crate) softness: f32,
}

impl<'a> IkConstraintData<'a> {
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
