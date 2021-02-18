use crate::attachments::atlas_attachment_loader::AtlasAttachmentLoader;
use crate::attachments::attachment_loader::AttachmentLoader;
use crate::attachments::mesh_attachment::MeshAttachment;
use crate::skeleton_data::SkeletonData;
use crate::utils::color::Color;
use crate::utils::linked_mesh::LinkedMesh;
use crate::utils::texture_atlas::TextureAtlas;

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

pub struct SkeletonBinary {
    attachment_loader: Box<dyn AttachmentLoader>,
    linked_meshes: Vec<LinkedMesh>,
    scale: f32,
    temp_color1: Color,
    temp_color2: Color,
}

impl SkeletonBinary {
    pub fn new(atlas: TextureAtlas) -> Self {
        SkeletonBinary {
            attachment_loader: Box::new(AtlasAttachmentLoader::new(atlas)),
            linked_meshes: vec![],
            scale: 1.0,
            temp_color1: Default::default(),
            temp_color2: Default::default()
        }
    }

    // pub fn read_skeleton_data(&self, file: String) -> SkeletonData {
    //     let scale = self.scale;
    //     let skeletonData = SkeletonData::new;
    //     return skeletonData
    // }
}
