use super::*;

#[path = "./links/mod.rs"]
mod links;
use links::*;

pub fn ml_body(stack: &Stack) -> Box {
    let box_body = Box::new(Orientation::Vertical, 5);
    box_body.add_css_class("box_body_ml");

    let section_ml = Label::new(Some("Suporte"));
    section_ml.add_css_class("section_ml");

    box_body.append(&links(stack));

    box_body.set_valign(gtk::Align::Center);

    box_body
}
