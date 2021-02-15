use crate::utils::color::Color;

pub struct BoneData {
    index: i32,
    name: String,
    parent: *const BoneData,
    color: Color,
    length: f32,
    x: f32,
    y: f32,
    rotation: f32,
    scaleX: f32,
    scaleY: f32,
    shearX: f32,
    shearY: f32,
    transformMode: TransformMode,
    skinRequired: bool,
}

impl BoneData {
    pub fn new(index: i32, name: String, parent: *const BoneData) -> BoneData {
        BoneData {
            index,
            name,
            parent,
            color: Color{
                r: 0.61,
                g: 0.61,
                b: 0.61,
                a: 0.61
            },
            length: 0.0,
            x: 0.0,
            y: 0.0,
            rotation: 0.0,
            scaleX: 0.0,
            scaleY: 0.0,
            shearX: 0.0,
            shearY: 0.0,
            transformMode: TransformMode::Normal,
            skinRequired: false
        }
    }
}

pub enum TransformMode {
    Normal,
    OnlyTranslation,
    NoRotationOrReflection,
    NoScale,
    NoScaleOrReflection,
}

impl TransformMode {
    pub fn values(value: i32) -> TransformMode {
        match value {
            0 => TransformMode::Normal,
            1 => TransformMode::OnlyTranslation,
            2 => TransformMode::NoRotationOrReflection,
            3 => TransformMode::NoScale,
            4 => TransformMode::NoScaleOrReflection,
            _ => TransformMode::Normal,
        }
    }
}