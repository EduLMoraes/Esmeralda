use super::*;
use crate::env::var;

pub fn ml_head() -> Box {
    let box_head = Box::new(Orientation::Vertical, 0);
    box_head.add_css_class("box_head_ml");
    let version = Label::new(Some(&format!(
        "Versão {}",
        std::env::var("CARGO_PKG_VERSION").unwrap()
    )));
    version.add_css_class("version");

    let box_title = Box::new(Orientation::Horizontal, 0);
    box_title.add_css_class("box_title_ml");

    let icon = Image::from_file(format!("{}icon.png", var("ICON_PATH").unwrap()));
    icon.add_css_class("icon_ml");

    let title = Label::new(Some("Esmeralda"));
    title.add_css_class("title_ml");

    box_title.append(&icon);
    box_title.append(&title);
    box_title.append(&version);

    box_title.set_halign(gtk::Align::Center);

    let menu = Label::new(Some("Menu"));
    menu.add_css_class("section_ml");

    box_head.append(&box_title);
    box_head.append(&menu);

    box_head
}
