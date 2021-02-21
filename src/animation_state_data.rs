use crate::skeleton_data::SkeletonData;
use std::collections::HashMap;
use crate::animation::Animation;

pub struct AnimationStateData<'a> {
    skeletonData: &'a SkeletonData<'a>,
    animationToMixTime: HashMap<Key<'a>, f32>,
    tempKey: Key<'a>,
    defaultMix: f32,
}

impl<'a> AnimationStateData<'a> {
    pub fn new(skeletonData: &'a SkeletonData<'a>) -> Self {
        Self {
            skeletonData,
            animationToMixTime: Default::default(),
            tempKey: Key::new(),
            defaultMix: 0.0
        }
    }
}

struct Key<'a> {
    a1: Option<&'a Animation>,
    a2: Option<&'a Animation>,
}

impl<'a> Key<'a> {
    pub fn new() -> Self {
        Self {
            a1: None,
            a2: None
        }
    }
}