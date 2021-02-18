use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use object_pool::Pool;
use phf::{Map, PhfHash};

use crate::attachments::attachment::Attachment;
use crate::bone_data::BoneData;
use crate::constraint_data::ConstraintData;

pub struct Skin<'b, 'c> {
    pub(crate) name: String,
    attachments: Map<SkinEntry, SkinEntry>,
    bones: Vec<&'b BoneData<'b>>,
    constraints: Vec<&'c ConstraintData>,
    keyPool: Pool<Key>,
    lookup: SkinEntry,
}

impl<'b, 'c> Skin<'b, 'c> {}

pub struct SkinEntry {
    slotIndex: i32,
    name: String,
    attachment: Attachment,
    hashCode: i32,
}

impl SkinEntry {
    pub fn new() -> Self {
        return SkinEntry::set(0, "".to_string());
    }

    pub fn with(slotIndex: i32, name: String, attachment: Attachment) -> Self {
        let mut i = SkinEntry::set(slotIndex, name);
        i.attachment = attachment;
        return i;
    }

    fn set(slotIndex: i32, name: String) -> Self {
        if name.is_empty() { panic!("name cannot be null.") }
        let mut hasher = DefaultHasher::new();
        name.phf_hash(&mut hasher);
        SkinEntry {
            slotIndex,
            name,
            attachment: Attachment::new("".to_string()),
            hashCode: hasher.finish() as i32 + slotIndex + 37,
        }
    }
}

impl PartialEq for SkinEntry {
    fn eq(&self, other: &Self) -> bool {
        if self.slotIndex != other.slotIndex { return false; };
        return self.name.eq(&other.name);
    }
}

struct Key {
    slotIndex: i32,
    name: String,
    hashCode: i32,
}

impl Key {}
