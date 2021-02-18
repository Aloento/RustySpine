#![allow(non_snake_case)]
#![feature(assoc_char_funcs)]
mod attachments;
mod blend_mode;
mod bone_data;
mod skeleton_binary;
mod skeleton_data;
mod skin;
mod slot_data;
mod utils;
mod event_data;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
