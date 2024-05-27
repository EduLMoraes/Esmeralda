use super::*;

#[path = "./links/mod.rs"]
mod links;
use links::*;

pub fn ml_body(stack: &Stack) -> Box {
    let box_body = Box::new(Orientation::Vertical, 5);
    box_body.add_css_class("box_body_ml");

    let test = Button::with_label("Atualizar");
    test.connect_clicked(|_| {
        use crate::utils::update::update_sys;
        println!("atualizando...");
        update_sys();
        println!("atualizado!");
    });

    box_body.append(&links(stack));
    box_body.append(&test);

    box_body.set_valign(gtk::Align::Center);

    box_body
}
