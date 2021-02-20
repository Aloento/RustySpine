use crate::bone_data::BoneData;
use crate::skeleton::Skeleton;

pub struct Bone<'a> {
    data: &'a BoneData<'a>,
    pub(crate) skeleton: &'a Skeleton<'a>,
    parent: Option<&'a Bone<'a>>,
    children: Vec<Bone<'a>>,
    appliedValid: bool,
    sorted: bool,
    active: bool,
    x: f32,
    y: f32,
    rotation: f32,
    scaleX: f32,
    scaleY: f32,
    shearX: f32,
    shearY: f32,
    ax: f32,
    ay: f32,
    arotation: f32,
    ascaleX: f32,
    ascaleY: f32,
    ashearX: f32,
    ashearY: f32,
    a: f32,
    b: f32,
    worldX: f32,
    c: f32,
    d: f32,
    worldY: f32,
}

impl<'a> Bone<'a> {
    pub fn new(data: &BoneData, skeleton: &Skeleton, parent: Option<&Bone>) -> Self {
        let mut i = Bone {
            data,
            skeleton,
            parent,
            children: vec![],
            appliedValid: false,
            sorted: false,
            active: false,
            x: 0.0,
            y: 0.0,
            rotation: 0.0,
            scaleX: 0.0,
            scaleY: 0.0,
            shearX: 0.0,
            shearY: 0.0,
            ax: 0.0,
            ay: 0.0,
            arotation: 0.0,
            ascaleX: 0.0,
            ascaleY: 0.0,
            ashearX: 0.0,
            ashearY: 0.0,
            a: 0.0,
            b: 0.0,
            worldX: 0.0,
            c: 0.0,
            d: 0.0,
            worldY: 0.0,
        };
        i.set_to_setup_pose();
        return i;
    }

    pub fn set_to_setup_pose(&mut self) {
        let data = self.data;
        self.x = data.x;
        self.y = data.y;
        self.rotation = data.rotation;
        self.scaleX = data.scaleX;
        self.scaleY = data.scaleY;
        self.shearX = data.shearX;
        self.shearY = data.shearY;
    }
}
