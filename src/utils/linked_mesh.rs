use crate::attachments::mesh_attachment::MeshAttachment;

pub struct LinkedMesh<'a> {
    parent: String,
    skin: String,
    slot_index: i32,
    mesh: &'a MeshAttachment<'a>,
    inherit_deform: bool,
}

impl<'a> LinkedMesh<'a> {
    fn new(
        mesh: &'a MeshAttachment,
        skin: String,
        slot_index: i32,
        parent: String,
        inherit_deform: bool,
    ) -> Self {
        LinkedMesh {
            parent,
            skin,
            slot_index,
            mesh,
            inherit_deform,
        }
    }
}
