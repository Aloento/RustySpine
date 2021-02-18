use crate::constraint_data::ConstraintData;
use crate::bone_data::BoneData;

pub struct TransformConstraintData<'b> {
    data: ConstraintData,
    bones: Vec<&'b BoneData<'b>>,
    target: Option<&'b BoneData<'b>>,
    rotateMix: f32,
    translateMix: f32,
    scaleMix: f32,
    shearMix: f32,
    offsetRotation: f32,
    offsetX: f32,
    offsetY: f32,
    offsetScaleX: f32,
    offsetScaleY: f32,
    offsetShearY: f32,
    relative: bool,
    local: bool,
}

impl<'b> TransformConstraintData<'b> {
    pub fn new(name: String) -> Self {
        TransformConstraintData{
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
            local: false
        }
    }
}
