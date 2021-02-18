use crate::skeleton::Skeleton;
use crate::event::Event;
use batsat::intmap::IntSet;
use batsat::Lit;

pub struct Animation {
    name: String,
    timelines: Vec<Box<dyn Timeline>>,
    timelineIDs: IntSet<Lit>,
    duration: f32,
}

pub enum MixBlend {
    Setup,
    First,
    Replace,
    Add
}

pub enum MixDirection {
    In,
    Out
}

pub trait Timeline {
    fn apply(&self, skeleton: Skeleton, lastTime: f32, events: Vec<Event>, alpha: f32, blend: MixBlend, direction: MixDirection);
}
