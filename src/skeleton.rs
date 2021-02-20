use crate::slot::Slot;
use crate::skeleton_data::SkeletonData;
use crate::bone::Bone;
use crate::ik_constraint::IkConstraint;
use crate::path_constraint::PathConstraint;
use crate::updatable::Updatable;
use crate::utils::color::Color;
use crate::skin::Skin;
use crate::transform_constraint::TransformConstraint;

pub struct Skeleton<'a> {
    data: SkeletonData<'a>,
    bones: Vec<Bone<'a>>,
    pub(crate) slots: Vec<Slot<'a>>,
    ikConstraints: Vec<IkConstraint<'a>>,
    transformConstraints: Vec<TransformConstraint<'a>>,
    pathConstraints: Vec<PathConstraint<'a>>,
    updateCache: Vec<Box<dyn Updatable>>,
    updateCacheReset: Vec<&'a Bone<'a>>,
    color: Color,
    drawOrder: Vec<Slot<'a>>,
    skin: Option<&'a Skin<'a>>,
    pub(crate) time: f32,
    scaleX: f32,
    scaleY: f32,
    x: f32,
    y: f32,
}

impl<'a> Skeleton<'a> {
    pub fn new(data: SkeletonData) -> Self {
        let mut i = Self {
            data,
            bones: Vec::with_capacity(data.bones.len()),
            slots: Vec::with_capacity(data.slots.len()),
            ikConstraints: Vec::with_capacity(data.ikConstraints.len()),
            transformConstraints: Vec::with_capacity(data.transformConstraints.len()),
            pathConstraints: Vec::with_capacity(data.pathConstraints.len()),
            updateCache: vec![],
            updateCacheReset: vec![],
            color: Color{
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: 1.0
            },
            drawOrder: Vec::with_capacity(data.slots.len()),
            skin: None,
            time: 0.0,
            scaleX: 1.0,
            scaleY: 1.0,
            x: 0.0,
            y: 0.0
        };

        return i;
    }
}
