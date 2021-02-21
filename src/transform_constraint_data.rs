use crate::bone_data::BoneData;
use crate::constraint_data::ConstraintData;

pub struct TransformConstraintData<'a> {
    data: ConstraintData,
    pub(crate) bones: Vec<&'a BoneData<'a>>,
    pub(crate) target: Option<&'a BoneData<'a>>,
    pub(crate) rotateMix: f32,
    pub(crate) translateMix: f32,
    pub(crate) scaleMix: f32,
    pub(crate) shearMix: f32,
    offsetRotation: f32,
    offsetX: f32,
    offsetY: f32,
    offsetScaleX: f32,
    offsetScaleY: f32,
    offsetShearY: f32,
    relative: bool,
    local: bool,
}

impl<'a> TransformConstraintData<'a> {
    pub fn new(name: String) -> Self {
        TransformConstraintData {
            data: ConstraintData::new(name),
            bones: vec![],
            target: None,
            rotateMix: 0.0,
            translateMix: 0.0,
            scaleMix: 0.0,
            shearMix: 0.0,
            offsetRotation: 0.0,
            offsetX: 0.0,
            offsetY: 0.0,
            offsetScaleX: 0.0,
            offsetScaleY: 0.0,
            offsetShearY: 0.0,
            relative: false,
            local: false,
        }
    }
}
