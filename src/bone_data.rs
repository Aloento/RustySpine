use crate::utils::color::Color;

pub struct BoneData<'a> {
    index: i32,
    pub(crate) name: String,
    parent: Option<&'a BoneData<'a>>,
    color: Color,
    length: f32,
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) rotation: f32,
    pub(crate) scaleX: f32,
    pub(crate) scaleY: f32,
    pub(crate) shearX: f32,
    pub(crate) shearY: f32,
    transformMode: TransformMode,
    skinRequired: bool,
}

impl<'a> BoneData<'a> {
    pub fn new(index: i32, name: String, parent: Option<&'a BoneData<'a>>) -> Self {
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
