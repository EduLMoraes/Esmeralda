use super::*;
use gtk::Image;
use std::path::PathBuf;


mod bottom;
mod top;

pub use bottom::*;
pub use top::*;

pub fn get_box_body(stack: &Stack) -> Box {
    let box_body = Box::new(Orientation::Vertical, 0);

    stack.add_titled(&box_count(), Some("Contas"), "Contas");
    stack.add_titled(&box_graph(), Some("Graficos"), "Graficos");

    box_body.append(&box_top(stack));
    box_body.append(stack);

    box_body.add_css_class("box_body");

    box_body
}
