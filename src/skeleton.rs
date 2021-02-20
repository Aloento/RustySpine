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
