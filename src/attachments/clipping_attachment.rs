use crate::attachments::vertex_attachment::VertexAttachment;
use crate::slot_data::SlotData;
use crate::utils::color::Color;

pub struct ClippingAttachment<'a> {
    Vertex: VertexAttachment<'a>,
    color: Color,
    endSlot: Option<&'a SlotData<'a>>,
}

impl<'a> ClippingAttachment<'a> {
    pub fn new(name: String) -> Self {
        Self {
            Vertex: VertexAttachment::new(name),
            color: Color {
                r: 0.2275,
                g: 0.2275,
                b: 0.8078,
                a: 1.0,
            },
            endSlot: None,
        }
    }
}
