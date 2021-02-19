use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use crate::attachments::attachment::Attachment;
use crate::bone_data::BoneData;
use crate::constraint_data::ConstraintData;
use crate::skeleton::Skeleton;
use std::ops::Deref;

pub struct Skin<'a> {
    pub(crate) name: String,
    attachments: HashMap<SkinEntry, SkinEntry>,
    bones: Vec<&'a BoneData<'a>>,
    constraints: Vec<&'a ConstraintData>,
    lookup: SkinEntry,
}

impl<'a> Skin<'a> {
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

    pub fn set_attachment(&mut self, slotIndex: i32, name: String, attachment: Option<Attachment>) {
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

    pub fn get_attachment(&mut self, slotIndex: i32, name: &String) -> Option<&Attachment> {
        self.lookup.set(slotIndex, name.clone());
        let entry = self.attachments.get(&self.lookup);
        match entry {
            None => None,
            Some(entry) => Some(&entry.attachment.unwrap()),
        }
    }

    fn attachAll(&mut self, skeleton: &mut Skeleton<'a>, oldSkin: &Skin) {
        for entry in oldSkin.attachments.keys() {
            let slotIndex = entry.slotIndex;
            let slot = skeleton.slots.get_mut(slotIndex as usize).unwrap();
            if slot.attachment == &entry.attachment.unwrap() {
                let attachment = self.get_attachment(slotIndex, &entry.name);
                match attachment {
                    None => {}
                    Some(_) => { slot.set_attachment(attachment.unwrap()) }
                }
            }
        }
    }
}

#[derive(Clone, Eq, Hash)]
pub struct SkinEntry {
    slotIndex: i32,
    name: String,
    attachment: Option<Attachment>,
    hashCode: i32,
}

impl SkinEntry {
    pub fn new() -> Self {
        return Self::with(0, "".to_string(), None);
    }

    pub fn with(slotIndex: i32, name: String, attachment: Option<Attachment>) -> Self {
        let mut hasher = DefaultHasher::new();
        name.hash(&mut hasher);
        Self {
            slotIndex,
            name,
            attachment,
            hashCode: hasher.finish() as i32 + slotIndex + 37,
        }
    }

    fn set(&mut self, slotIndex: i32, name: String) {
        if name.is_empty() { panic!("name cannot be null.") }
        let mut hasher = DefaultHasher::new();
        name.hash(&mut hasher);
        self.name = name;
        self.hashCode = hasher.finish() as i32 + slotIndex + 37;
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
