use std::collections::HashSet;
use crate::attachments::texture::Texture;
use crate::attachments::texture_region::TextureRegion;
use std::fs::File;

pub struct TextureAtlas {
    tuple: Vec<String>,
    textures: HashSet<Texture>,
    regions: Vec<AtlasRegion>,
    Data: TextureAtlasData,
}

pub struct TextureAtlasData {
    pages: Vec<Page>,
    regions: Vec<Region>,
    Page: Page,
    Region: Region,
}

pub struct Page {
    textureFile: File,
    texture: Texture,
    width: f32,
    height: f32,
    useMipMaps: bool,
    format: Format,
    minFilter: TextureFilter,
    magFilter: TextureFilter,
    uWrap: TextureWrap,
    vWrap: TextureWrap,
}

pub struct Region {
    page: Page,
    index: i32,
    name: String,
    offsetX: f32,
    offsetY: f32,
    originalWidth: i32,
    originalHeight: i32,
    rotate: bool,
    degrees: i32,
    left: i32,
    top: i32,
    width: i32,
    height: i32,
    flip: bool,
    splits: Vec<i32>,
    pads: Vec<i32>,
}

pub struct AtlasRegion {
    Region: TextureRegion,
    index: i32,
    name: String,
    offsetX: f32,
    offsetY: f32,
    packedWidth: i32,
    packedHeight: i32,
    originalWidth: i32,
    originalHeight: i32,
    rotate: bool,
    degrees: i32,
    splits: Vec<i32>,
    pads: Vec<i32>,
}
