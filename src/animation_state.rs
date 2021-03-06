use object_pool::Pool;

use crate::animation::{Animation, MixBlend};
use crate::animation_state_data::AnimationStateData;
use crate::event::Event;

const SUBSEQUENT: i32 = 0;
const FIRST: i32 = 1;
const HOLD: i32 = 2;
const HOLD_MIX: i32 = 3;
const SETUP: i32 = 1;
const CURRENT: i32 = 2;

pub struct AnimationState<'a> {
    emptyAnimation: Animation,
    tracks: Vec<TrackEntry<'a>>,
    listeners: Vec<Box<dyn AnimationStateListener>>,
    trackEntryPool: Pool<TrackEntry<'a>>,
    events: Vec<Event>,
    queue: EventQueue,
    propertyIDs: Vec<i32>,
    animationsChanged: bool,
    data: AnimationStateData<'a>,
    timeScale: f32,
    unkeyedState: i32,
}

impl<'a> AnimationState<'a> {
    pub fn new(data: AnimationStateData) -> Self {
        Self {
            emptyAnimation: Animation::new("<empty>".to_string(), Vec::with_capacity(0), 0.0),
            tracks: vec![],
            listeners: vec![],
            trackEntryPool: Pool::new(32, TrackEntry::new()),
            events: vec![],
            queue: EventQueue {},
            propertyIDs: vec![],
            animationsChanged: false,
            data,
            timeScale: -1.0,
            unkeyedState: 0,
        }
    }
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

impl TrackEntry {
    pub fn new() -> Self {
        Self {
            timelineMode: vec![],
            timelineHoldMix: vec![],
            timelinesRotation: vec![],
            animation: None,
            next: None,
            mixingFrom: None,
            mixingTo: None,
            listener: (),
            trackIndex: 0,
            Loop: false,
            holdPrevious: false,
            eventThreshold: 0.0,
            attachmentThreshold: 0.0,
            drawOrderThreshold: 0.0,
            animationStart: 0.0,
            animationEnd: 0.0,
            animationLast: 0.0,
            nextAnimationLast: 0.0,
            delay: 0.0,
            trackTime: 0.0,
            trackLast: 0.0,
            nextTrackLast: 0.0,
            trackEnd: 0.0,
            timeScale: 0.0,
            alpha: 0.0,
            mixTime: 0.0,
            mixDuration: 0.0,
            interruptAlpha: 0.0,
            totalAlpha: 0.0,
            mixBlend: MixBlend::Replace,
        }
    }

    pub fn reset(&mut self) {
        self.next = None,
        self.mixingFrom = None,
        self.mixingTo = None,
        self.animation = None,
        self.listener = None,
        self.timelineMode.clear();
        self.timelineHoldMix.clear();
        self.timelinesRotation.clear();
    }
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

struct EventQueue {}
