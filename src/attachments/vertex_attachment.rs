use crate::attachments::attachment::Attachment;

pub struct VertexAttachment<'a> {
    Attachment: Attachment,
    nextID: i32,
    id: i32,
    bones: Vec<i32>,
    vertices: Vec<f32>,
    worldVerticesLength: i32,
    deformAttachment: Option<&'a VertexAttachment<'a>>,
}

impl VertexAttachment {
    pub fn new(name: String) -> Self {
        let mut i = Self {
            Attachment: Attachment::new(name),
            nextID: 0,
            id: 0,
            bones: vec![],
            vertices: vec![],
            worldVerticesLength: 0,
            deformAttachment: None,
        };
        i.deformAttachment = Some(&i);
        i.id = (i.nextID & 65535) << 11;
        return i;
    }
}
