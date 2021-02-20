use crate::attachments::attachment::Attachment;
use crate::slot_data::SlotData;

pub struct Slot<'a> {
    data: &'a SlotData<'a>,
    bone: &'a Bone<'a>,

    pub(crate) attachment: &'a Attachment,
}

impl<'a> Slot<'a> {
    pub fn set_attachment(&mut self, attachment: &'a Attachment) {
        if self.attachment == attachment {
            return;
        }
        self.attachment = attachment;
    }
}
