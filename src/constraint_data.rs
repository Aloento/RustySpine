pub struct ConstraintData {
    name: String,
    order: i32,
    skin_required: bool,
}

impl ConstraintData {
    pub fn new(name: String) -> Self {
        ConstraintData {
            name,
            order: 0,
            skin_required: false,
        }
    }
}
