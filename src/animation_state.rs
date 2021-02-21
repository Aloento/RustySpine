use crate::animation_state_data::AnimationStateData;
use crate::animation::{Animation, MixBlend};
use crate::event::Event;
use object_pool::Pool;

const SUBSEQUENT: i32 = 0;
const FIRST: i32 = 1;
const HOLD: i32 = 2;
const HOLD_MIX: i32 = 3;
const SETUP: i32 = 1;
const CURRENT: i32 = 2;

pub struct AnimationState<'a> {
    emptyAnimation: Animation,
    data: AnimationStateData<'a>,
    tracks: Vec<TrackEntry<'a>>,
    listeners: Vec<Box<dyn AnimationStateListener>>,
    trackEntryPool: Pool<&'a TrackEntry<'a>>,
    events: Vec<Event>,
    queue:
}

pub struct TrackEntry<'a> {
    timelineMode: Vec<i32>,
    timelineHoldMix: Vec<&'a TrackEntry<'a>>,
    timelinesRotation: Vec<f32>,
    animation: Option<&'a Animation>,
    next: Option<&'a TrackEntry<'a>>,
    mixingFrom: Option<&'a TrackEntry<'a>>,
    mixingTo: Option<&'a TrackEntry<'a>>,
    listener: dyn AnimationStateListener,
    trackIndex: i32,
    Loop: bool,
    holdPrevious: bool,
    eventThreshold: f32,
    attachmentThreshold: f32,
    drawOrderThreshold: f32,
    animationStart: f32,
    animationEnd: f32,
    animationLast: f32,
    nextAnimationLast: f32,
    delay: f32,
    trackTime: f32,
    trackLast: f32,
    nextTrackLast: f32,
    trackEnd: f32,
    timeScale: f32,
    alpha: f32,
    mixTime: f32,
    mixDuration: f32,
    interruptAlpha: f32,
    totalAlpha: f32,
    mixBlend: MixBlend,
}

pub trait AnimationStateListener {
    fn start(entry: TrackEntry);
    fn interrupt(entry: TrackEntry);
    fn end(entry: TrackEntry);
    fn dispose(entry: TrackEntry);
    fn complete(entry: TrackEntry);
    fn event(entry: TrackEntry, event: Event);

    fn eventInt(trackIndex: i32, event: Event);
    fn completeInt(trackIndex: i32, loopCount: i32);
    fn startInt(trackIndex: i32);
    fn endInt(trackIndex: i32);
}

struct EventQueue {

}
