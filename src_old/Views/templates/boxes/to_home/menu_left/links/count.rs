use super::*;
use crate::env::var;

pub fn box_count_link(stack: &Stack) -> Box {
    let fbox_count = Box::new(Orientation::Horizontal, 0);
    let count_icon = Image::from_file(format!("{}counts.png", var("ICON_PATH").unwrap()));
    let count_link = Button::with_label("Contas");
    count_link.set_css_classes(&["link_view"]);

    count_link.connect_clicked(clone!(
        #[weak]
        stack,
        move |_| {
            reload_home(None, Some(&stack));
            stack.set_visible_child_name("Contas");
        }
    ));

    count_icon.add_css_class("icon_ml");

    fbox_count.append(&count_icon);
    fbox_count.append(&count_link);
    fbox_count.set_valign(gtk::Align::Center);

    fbox_count
}
