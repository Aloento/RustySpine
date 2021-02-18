use crate::attachments::mesh_attachment::MeshAttachment;

pub struct LinkedMesh<'m> {
    parent: String,
    skin: String,
    slot_index: i32,
    mesh: &'m MeshAttachment,
    inherit_deform: bool,
}

impl<'m> LinkedMesh<'m> {
    fn new(mesh: &'m MeshAttachment, skin: String, slot_index: i32, parent: String, inherit_deform: bool)
           -> Self {
        LinkedMesh {
            parent,
            skin,
            slot_index,
            mesh,
            inherit_deform,
        }
    }
}
