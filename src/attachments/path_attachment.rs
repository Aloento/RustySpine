use crate::attachments::vertex_attachment::VertexAttachment;
use crate::utils::color::Color;

pub struct PathAttachment<'a> {
    Vertex: VertexAttachment<'a>,
    color: Color,
    lengths: Vec<f32>,
    closed: bool,
    constantSpeed: bool,
}

impl PathAttachment {
    pub fn new(name: String) -> Self {
        Self {
            Vertex: VertexAttachment::new(name),
            color: Color {
                r: 1.0,
                g: 0.5,
                b: 0.0,
                a: 1.0,
            },
            lengths: vec![],
            closed: false,
            constantSpeed: false,
        }
    }
}
