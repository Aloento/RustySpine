use crate::attachments::texture_region::TextureRegion;
use crate::attachments::vertex_attachment::VertexAttachment;
use crate::utils::color::Color;

pub struct MeshAttachment<'a> {
    Vertex: VertexAttachment<'a>,
    color: Color,
    region: TextureRegion,
    path: String,
    regionUVs: Vec<f32>,
    uvs: Vec<f32>,
    triangles: Vec<i16>,
    hullLength: i32,
    parentMesh: Option<&'a MeshAttachment<'a>>,
    edges: Vec<i16>,
    width: f32,
    height: f32,
}

impl<'a> MeshAttachment<'a> {
    pub fn new(name: String) -> Self {
        Self {
            Vertex: VertexAttachment::new(name),
            color: Color::new(),
            region: TextureRegion {},
            path: "".to_string(),
            regionUVs: vec![],
            uvs: vec![],
            triangles: vec![],
            hullLength: 0,
            parentMesh: None,
            edges: vec![],
            width: 0.0,
            height: 0.0,
        }
    }
}
