use super::*;
use crate::env::var;
use gtk::{Adjustment, CheckButton, SpinButton};

#[path = "./boxes/mod.rs"]
mod boxes;
use boxes::to_register::*;

pub fn rgter_screen(stack: &Stack) -> Box {
    let screen = Box::new(Orientation::Vertical, 26);

    let box_register = box_register(stack);

    let return_button = LinkButton::new("Voltar para login");
    let return_image = Image::from_file(format!("{}return.png", var("IMG_PATH").unwrap()));
    let box_return = Box::new(Orientation::Horizontal, 0);
    box_return.append(&return_image);
    box_return.append(&return_button);

    return_button.remove_css_class("link");

    return_image.add_css_class("return_img_register");
    return_button.add_css_class("return_register");
    box_return.add_css_class("box_return");

    screen.append(&box_return);
    screen.append(&box_register);

    return_button.connect_clicked(clone!(@weak stack => move |_| {
        stack.remove_css_class("register_window");
        stack.add_css_class("login_window");
        stack.set_visible_child_name("login");
    }));

    screen
}
