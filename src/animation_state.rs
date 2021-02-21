use crate::animation_state_data::AnimationStateData;
use crate::animation::Animation;
use crate::event::Event;

const SUBSEQUENT: i32 = 0;
const FIRST: i32 = 1;
const HOLD: i32 = 2;
const HOLD_MIX: i32 = 3;
const SETUP: i32 = 1;
const CURRENT: i32 = 2;

pub struct AnimationState<'a> {
    emptyAnimation: Animation,
    data: AnimationStateData<'a>,
    tracks: Vec<TrackEntry>
}

pub struct TrackEntry<'a> {
    timelineMode: Vec<i32>,
    timelineHoldMix: Vec<&'a TrackEntry<'a>>,
    timelinesRotation: Vec<f32>,
    animation: Option<&'a Animation>,
    next: Option<&'a TrackEntry<'a>>,
    mixingFrom: Option<&'a TrackEntry<'a>>,
    mixingTo: Option<&'a TrackEntry<'a>>,
    listener:

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
}fn
