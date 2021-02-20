use crate::bone::Bone;

pub struct IkConstraint<'a> {
    bones: Vec<&'a Bone<'a>>,

}
