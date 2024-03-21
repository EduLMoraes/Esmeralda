use super::*;
use crate::env::var;

mod form_left;
mod form_right;
pub use form_left::*;
pub use form_right::*;

pub fn box_register() -> Box {
    let box_register = Box::new(Orientation::Vertical, 26);

    let box_title = Box::new(Orientation::Vertical, 0);
    let title = Label::new(Some("Cadastro"));
    let img = Image::from_file(format!("{}perfil-photo.png", var("IMG_PATH").unwrap()));

    box_title.append(&img);
    box_title.append(&title);
    box_title.set_halign(gtk::Align::Center);

    let box_form_main = Box::new(Orientation::Horizontal, 0);
    let box_form_left = form_left();
    let box_form_right = form_right();

    box_form_main.append(&box_form_left);
    box_form_main.append(&box_form_right);

    let login_button = Button::with_label("Confirmar");

    box_register.set_halign(gtk::Align::Center);
    box_register.set_valign(gtk::Align::Center);
    box_register.set_size_request(450, 450);
    login_button.set_halign(gtk::Align::End);

    box_register.append(&box_title);
    box_form_right.append(&login_button);
    box_register.append(&box_form_main);

    login_button.add_css_class("button");
    title.add_css_class("register_title");
    img.add_css_class("register_image");
    box_register.add_css_class("register_box");
    box_title.add_css_class("register_title_box");
    box_form_main.add_css_class("bf_main");

    box_register
}
