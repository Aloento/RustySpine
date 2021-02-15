use crate::attachments::mesh_attachment::MeshAttachment;
use crate::utils::color::Color;

const BONE_ROTATE: u8 = 0;
const BONE_TRANSLATE: u8 = 1;
const BONE_SCALE: u8 = 2;
const BONE_SHEAR: u8 = 3;
const SLOT_ATTACHMENT: u8 = 0;
const SLOT_COLOR: u8 = 1;
const SLOT_TWO_COLOR: u8 = 2;
const PATH_POSITION: u8 = 0;
const PATH_SPACING: u8 = 1;
const PATH_MIX: u8 = 2;
const CURVE_STEPPED: u8 = 1;
const CURVE_BEZIER: u8 = 2;
const TEMP_COLOR: Color = Color::new();
const TEMP_COLOR2: Color = Color::new();

pub struct SkeletonBinary {

}
