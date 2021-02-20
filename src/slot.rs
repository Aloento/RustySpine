use crate::attachments::attachment::Attachment;
use crate::bone::Bone;
use crate::slot_data::SlotData;
use crate::utils::color::Color;

pub struct Slot<'a> {
    data: &'a SlotData<'a>,
    bone: &'a Bone<'a>,
    color: Color,
    darkColor: Option<Color>,
    pub(crate) attachment: Option<&'a Attachment>,
    attachmentState: i32,
    attachmentTime: f32,
    deform: Vec<f32>,
}

impl<'a> Slot<'a> {
    pub fn new(data: &'a SlotData, bone: &'a Bone) -> Self {
        Slot {
            data,
            bone,
            color: Default::default(),
            darkColor: match data.darkColor {
                None => None,
                Some(_) => Default::default(),
            },
            attachment: None,
            attachmentState: 0,
            attachmentTime: 0.0,
            deform: vec![],
        }
    }

    pub fn set_attachment(&mut self, attachment: &'a Attachment) {
        if self.attachment.unwrap() == attachment { return; }
        self.attachment = Some(attachment);
        self.attachmentTime = self.bone.skeleton.time;
        self.deform.clear();
    }
}
