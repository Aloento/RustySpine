#![allow(non_snake_case)]
#![feature(assoc_char_funcs)]
#![feature(allocator_api)]
mod attachments;
mod blend_mode;
mod bone_data;
mod skeleton_binary;
mod skeleton_data;
mod skin;
mod slot_data;
mod utils;
mod event_data;
mod animation;
mod skeleton;
mod event;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
