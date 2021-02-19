#![allow(non_snake_case)]
#![feature(assoc_char_funcs)]
#![feature(allocator_api)]

mod animation;
mod attachments;
mod blend_mode;
mod bone_data;
mod constraint_data;
mod event;
mod event_data;
mod ik_constraint_data;
mod path_constraint_data;
mod skeleton;
mod skeleton_binary;
mod skeleton_data;
mod skin;
mod slot;
mod slot_data;
mod transform_constraint_data;
mod utils;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
