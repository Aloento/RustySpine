use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;

use std::hash::{Hasher, Hash};

use crate::attachments::attachment::Attachment;
use crate::bone_data::BoneData;
use crate::constraint_data::ConstraintData;
use std::borrow::Borrow;
use std::cell::RefCell;

pub struct Skin<'b, 'c> {
    pub(crate) name: String,
    attachments: HashMap<SkinEntry, SkinEntry>,
    bones: Vec<&'b BoneData<'b>>,
    constraints: Vec<&'c ConstraintData>,
    lookup: SkinEntry,
}

impl<'b, 'c> Skin<'b, 'c> {
    pub fn new(name: String) -> Self {
        if name.is_empty() {
            panic!("name cannot be null.")
        };
        Self {
            name,
            attachments: HashMap::default(),
            bones: vec![],
            constraints: vec![],
            lookup: SkinEntry::new(),
        }
    }

    pub fn set_attachment(&mut self, slotIndex: i32, name: String, attachment: Attachment) {
        let mut newEntry = SkinEntry::with(slotIndex, name, attachment);
        let mut oldEntry = self.attachments.get_mut(&newEntry);
        match oldEntry {
            None => {
                self.attachments.insert(newEntry.clone(), newEntry);
            }
            Some(old) => {
                old.attachment = newEntry.attachment;
            }
        }
    }
}

#[derive(Clone, Eq, Hash)]
pub struct SkinEntry {
    slotIndex: i32,
    name: String,
    attachment: Attachment,
    hashCode: i32,
}

impl SkinEntry {
    pub fn new() -> Self {
        return Self::set(0, "".to_string());
    }

    pub fn with(slotIndex: i32, name: String, attachment: Attachment) -> Self {
        let mut i = Self::set(slotIndex, name);
        i.attachment = attachment;
        return i;
    }

    fn set(slotIndex: i32, name: String) -> Self {
        if name.is_empty() {
            panic!("name cannot be null.")
        }
        let mut hasher = DefaultHasher::new();
        name.hash(&mut hasher);
        Self {
            slotIndex,
            name,
            attachment: Attachment::new("".to_string()),
            hashCode: hasher.finish() as i32 + slotIndex + 37,
        }
    }
}

impl PartialEq for SkinEntry {
    fn eq(&self, other: &Self) -> bool {
        if self.slotIndex != other.slotIndex {
            return false;
        };
        return self.name.eq(&other.name);
    }
}
