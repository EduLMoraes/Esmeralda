use super::*;

pub fn box_investments_link(stack: &Stack) -> Box {
    let fbox_count = Box::new(Orientation::Horizontal, 0);
    let icon = Image::from_file(format!("{}investments.png", var("ICON_PATH").unwrap()));
    let link = Button::with_label("Investimentos");
    link.set_css_classes(&["link_view"]);

    link.connect_clicked(clone!(
        #[weak]
        stack,
        move |_| {
            reload_home(None, Some(&stack));
            stack.set_visible_child_name("Investimentos");
        }
    ));

    icon.add_css_class("icon_ml");

    fbox_count.append(&icon);
    fbox_count.append(&link);
    fbox_count.set_valign(gtk::Align::Center);

    fbox_count
}
