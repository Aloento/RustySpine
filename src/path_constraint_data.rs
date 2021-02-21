use crate::bone_data::BoneData;
use crate::constraint_data::ConstraintData;
use crate::slot_data::SlotData;

pub struct PathConstraintData<'a> {
    data: ConstraintData,
    pub(crate) bones: Vec<&'a BoneData<'a>>,
    pub(crate) target: Option<&'a SlotData<'a>>,
    positionMode: PositionMode,
    spacingMode: SpacingMode,
    rotateMode: RotateMode,
    offsetRotation: f32,
    pub(crate) position: f32,
    pub(crate) spacing: f32,
    pub(crate) rotateMix: f32,
    pub(crate) translateMix: f32,
}

impl<'a> PathConstraintData<'a> {
    pub fn new(name: String) -> Self {
        PathConstraintData {
            data: ConstraintData::new(name),
            bones: vec![],
            target: None,
            positionMode: PositionMode::Fixed,
            spacingMode: SpacingMode::Length,
            rotateMode: RotateMode::Tangent,
            offsetRotation: 0.0,
            position: 0.0,
            spacing: 0.0,
            rotateMix: 0.0,
            translateMix: 0.0,
        }
    }
}

pub enum PositionMode {
    Fixed,
    Percent,
}

impl PositionMode {
    pub fn values(value: i32) -> Self {
        match value {
            0 => PositionMode::Fixed,
            1 => PositionMode::Percent,
            _ => panic!("Invalid value for PositionMode: {}", value),
        }
    }
}

pub enum SpacingMode {
    Length,
    Fixed,
    Percent,
}

impl SpacingMode {
    pub fn values(value: i32) -> Self {
        match value {
            0 => SpacingMode::Length,
            1 => SpacingMode::Fixed,
            2 => SpacingMode::Percent,
            _ => panic!("Invalid value for SpacingMode: {}", value),
        }
    }
}

pub enum RotateMode {
    Tangent,
    Chain,
    ChainScale,
}

impl RotateMode {
    pub fn values(value: i32) -> Self {
        match value {
            0 => RotateMode::Tangent,
            1 => RotateMode::Chain,
            2 => RotateMode::ChainScale,
            _ => panic!("Invalid value for {}", value),
        }
    }
}
