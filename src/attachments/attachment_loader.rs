use crate::skin::Skin;
use crate::attachments::region_attachment::RegionAttachment;
use crate::attachments::mesh_attachment::MeshAttachment;
use crate::attachments::bounding_box_attachment::BoundingBoxAttachment;
use crate::attachments::clipping_attachment::ClippingAttachment;
use crate::attachments::path_attachment::PathAttachment;
use crate::attachments::point_attachment::PointAttachment;

pub trait AttachmentLoader {
    fn new_region_attachment(&self, skin: Skin, name: String, path: String) -> RegionAttachment;
    fn new_mesh_attachment(&self, skin: Skin, name: String, path: String) -> MeshAttachment;
    fn new_bounding_box_attachment(&self, skin: Skin, name: String) -> BoundingBoxAttachment;
    fn new_clipping_attachment(&self, skin: Skin, name: String) -> ClippingAttachment;
    fn new_path_attachment(&self, skin: Skin, name: String) -> PathAttachment;
    fn new_point_attachment(&self, skin: Skin, name: String) -> PointAttachment;
}
