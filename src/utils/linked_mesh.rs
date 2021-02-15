use crate::attachments::mesh_attachment::MeshAttachment;

pub struct LinkedMesh {
    parent: String,
    skin: String,
    slot_index: i32,
    mesh: MeshAttachment,
    inherit_deform: bool
}

impl LinkedMesh {
    fn new(mesh: MeshAttachment, skin: String, slot_index: i32, parent: String, inherit_deform: bool)
            -> Self {
        LinkedMesh {
            parent,
            skin,
            slot_index,
            mesh,
            inherit_deform
        }
    }
}
