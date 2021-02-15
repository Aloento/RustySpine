use crate::skin::Skin;
use crate::attachments::region_attachment::RegionAttachment;
use crate::attachments::mesh_attachment::MeshAttachment;
use crate::attachments::bounding_box_attachment::BoundingBoxAttachment;
use crate::attachments::clipping_attachment::ClippingAttachment;
use crate::attachments::path_attachment::PathAttachment;
use crate::attachments::point_attachment::PointAttachment;

pub trait AttachmentLoader {
    fn new_region_attachment(skin: Skin, name: str, path: str) -> RegionAttachment;
    fn new_mesh_attachment(skin: Skin, name: str, path: str) -> MeshAttachment;
    fn new_bounding_box_attachment(skin: Skin, name: str) -> BoundingBoxAttachment;
    fn new_clipping_attachment(skin: Skin, name: str) -> ClippingAttachment;
    fn new_path_attachment(skin: Skin, name: str) -> PathAttachment;
    fn new_point_attachment(skin: Skin, name: str) -> PointAttachment;
}
