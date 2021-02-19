use crate::attachments::attachment::Attachment;

pub struct Slot<'a> {
    pub(crate) attachment: &'a Attachment,
}

impl<'a> Slot<'a> {
    pub fn set_attachment(&mut self, attachment: &'a Attachment) {
        if self.attachment == attachment { return; }
        self.attachment = attachment;
    }
}
