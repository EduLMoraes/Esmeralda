use super::*;
#[path = "./magic_number.rs"]
mod magic_number;

#[allow(dead_code)]
pub fn get_investments_box() -> Box {
    let box_index = Box::new(Orientation::Vertical, 0);
    box_index.add_css_class("box_calculator_index");
    box_index.append(&magic_number::get_box());
    box_index
}
