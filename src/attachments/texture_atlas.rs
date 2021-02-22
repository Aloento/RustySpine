use std::collections::HashSet;
use std::fs::File;

use crate::attachments::texture::{Texture, TextureFilter, TextureWrap};
use crate::attachments::texture_region::TextureRegion;

pub struct TextureAtlas {
    tuple: Vec<String>,
    textures: HashSet<Texture>,
    regions: Vec<AtlasRegion>,
    Data: TextureAtlasData,
}

impl TextureAtlas {
    pub fn new() -> Self {
        Self {
            tuple: vec![],
            textures: Default::default(),
            regions: vec![],
            Data: TextureAtlasData::new(),
        }
    }
}

pub struct TextureAtlasData {
    pages: Vec<Page>,
    regions: Vec<Region>,
    Page: Page,
    Region: Region,
}

impl TextureAtlasData {
    pub fn new() -> Self {
        Self {
            pages: vec![],
            regions: vec![],
            Page: Page::new(),
            Region: Region::new(),
        }
    }
}

pub struct Page {
    textureFile: Option<File>,
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

impl Page {
    pub fn new() -> Self {
        Self {
            textureFile: None,
            texture: Texture {},
            width: 0.0,
            height: 0.0,
            useMipMaps: false,
            format: (),
            minFilter: TextureFilter::Nearest,
            magFilter: TextureFilter::Nearest,
            uWrap: TextureWrap::MirroredRepeat,
            vWrap: TextureWrap::MirroredRepeat,
        }
    }
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

impl Region {
    pub fn new() -> Self {
        Self {
            page: Page::new(),
            index: 0,
            name: "".to_string(),
            offsetX: 0.0,
            offsetY: 0.0,
            originalWidth: 0,
            originalHeight: 0,
            rotate: false,
            degrees: 0,
            left: 0,
            top: 0,
            width: 0,
            height: 0,
            flip: false,
            splits: vec![],
            pads: vec![],
        }
    }
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

impl AtlasRegion {
    pub fn new() -> Self {
        Self {
            Region: TextureRegion::new(),
            index: 0,
            name: "".to_string(),
            offsetX: 0.0,
            offsetY: 0.0,
            packedWidth: 0,
            packedHeight: 0,
            originalWidth: 0,
            originalHeight: 0,
            rotate: false,
            degrees: 0,
            splits: vec![],
            pads: vec![],
        }
    }
}
