use crate::attachments::attachment_loader::AttachmentLoader;
use crate::attachments::mesh_attachment::MeshAttachment;
use crate::attachments::clipping_attachment::ClippingAttachment;
use crate::skin::Skin;
use crate::attachments::point_attachment::PointAttachment;
use crate::attachments::bounding_box_attachment::BoundingBoxAttachment;
use crate::attachments::path_attachment::PathAttachment;
use crate::attachments::region_attachment::RegionAttachment;
use crate::utils::texture_atlas::TextureAtlas;

pub struct AtlasAttachmentLoader {
    atlas: TextureAtlas
}

impl AtlasAttachmentLoader {
    pub fn new(atlas: TextureAtlas) -> AtlasAttachmentLoader {
        AtlasAttachmentLoader {
            atlas
        }
    }
}

impl AttachmentLoader for AtlasAttachmentLoader {
    fn new_region_attachment(&self, skin: Skin, name: String, path: String) -> RegionAttachment {
        unimplemented!()
    }

    fn new_mesh_attachment(&self, skin: Skin, name: String, path: String) -> MeshAttachment {
        unimplemented!()
    }

    fn new_bounding_box_attachment(&self, skin: Skin, name: String) -> BoundingBoxAttachment {
        unimplemented!()
    }

    fn new_clipping_attachment(&self, skin: Skin, name: String) -> ClippingAttachment {
        unimplemented!()
    }

    fn new_path_attachment(&self, skin: Skin, name: String) -> PathAttachment {
        unimplemented!()
    }

    fn new_point_attachment(&self, skin: Skin, name: String) -> PointAttachment {
        unimplemented!()
    }
}
