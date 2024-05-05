use super::*;
use gtk::Image;

mod ml_body;
mod ml_head;

use ml_body::ml_body;
use ml_head::ml_head;

pub fn get_box_menu_left(stack: &Stack) -> Box {
    let box_ml = Box::new(Orientation::Vertical, 0);

    box_ml.append(&ml_head());
    box_ml.append(&ml_body(stack));
    box_ml.add_css_class("box_ml");

    box_ml
}
