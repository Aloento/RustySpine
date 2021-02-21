use std::collections::HashSet;

use crate::event::Event;
use crate::skeleton::Skeleton;

pub struct Animation {
    name: String,
    timelines: Vec<Box<dyn Timeline>>,
    timelineIDs: HashSet<i32>,
    duration: f32,
}

impl Animation {
    pub fn new(name: String, timelines: Vec<Box<dyn Timeline>>, duration: f32) -> Self {
        let mut i = Self {
            name,
            timelines,
            timelineIDs: Default::default(),
            duration
        };
        i.setTimelines();
        return i;
    }

    pub fn setTimelines(&mut self) {
        self.timelineIDs.clear();
        for timeline in self.timelines {
            self.timelineIDs.insert(timeline.getPropertyId());
        }
    }
}

pub enum MixBlend {
    Setup,
    First,
    Replace,
    Add,
}

pub enum MixDirection {
    In,
    Out,
}

pub trait Timeline {
    fn apply(&self, skeleton: Skeleton, lastTime: f32, events: Vec<Event>,
             alpha: f32, blend: MixBlend, direction: MixDirection,
    );

    fn getPropertyId(&self) -> i32;
}
