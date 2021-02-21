use crate::attachments::vertex_attachment::VertexAttachment;
use crate::utils::color::Color;

pub struct BoundingBoxAttachment<'a> {
    Vertex: VertexAttachment<'a>,
    color: Color,
}

impl<'a> BoundingBoxAttachment<'a> {
    pub fn new(name: String) -> Self {
        Self {
            Vertex: VertexAttachment::new(name),
            color: Color {
                r: 0.38,
                g: 0.94,
                b: 0.0,
                a: 1.0,
            },
        }
    }
}
