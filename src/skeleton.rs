use crate::slot::Slot;

pub struct Skeleton<'a> {
    pub(crate) slots: Vec<Slot<'a>>,
}
