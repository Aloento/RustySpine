use crate::utils::color::Color;

pub struct BoneData<'b> {
    index: i32,
    name: String,
    parent: Option<BoneData<'b>>,
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

impl<'b> BoneData<'b> {
    pub fn new(index: i32, name: String, parent: Option<BoneData<'b>>) -> Self {
        BoneData {
            index,
            name,
            parent,
            color: Color {
                r: 0.61,
                g: 0.61,
                b: 0.61,
                a: 0.61,
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
            skinRequired: false,
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
    pub fn values(value: i32) -> Self {
        match value {
            0 => TransformMode::Normal,
            1 => TransformMode::OnlyTranslation,
            2 => TransformMode::NoRotationOrReflection,
            3 => TransformMode::NoScale,
            4 => TransformMode::NoScaleOrReflection,
            _ => panic!("Invalid value for TransformMode: {}", value),
        }
    }
}
