use super::*:

pub fn get_box() -> Box{
    let box_index = Box::new(Orientation::Vertical, 0);
    box_index.add_css_class("box_calculator");
    box_index
}