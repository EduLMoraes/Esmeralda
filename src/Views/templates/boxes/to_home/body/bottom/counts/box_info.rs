use super::*;
use crate::model::Info::Info;

pub fn new_box_info(info: &Info) -> Box {
    let box_info = Box::new(Orientation::Horizontal, 0);
    box_info.add_css_class("box_new_info");

    let box_left_i = Box::new(Orientation::Vertical, 2);
    box_left_i.add_css_class("box_left_i");

    let name = Label::new(Some(&info.debtor));
    name.add_css_class("name_i");
    let mut icon_path = PathBuf::from(format!("{}info_icon", var("ICON_PATH").unwrap()));
    icon_path.push(format!("{}.png", info.nature.to_lowercase()));

    let icon = Image::from_file(icon_path);
    icon.add_css_class("icon_info");

    if info.status {
        icon.add_css_class("positive");
    } else {
        icon.add_css_class("negative");
    }

    box_left_i.append(&name);
    box_left_i.append(&icon);
    box_left_i.set_halign(gtk::Align::Start);

    let box_center_i = Box::new(Orientation::Vertical, 2);
    box_center_i.add_css_class("box_center_i");
    box_center_i.set_valign(gtk::Align::Center);

    let value = format!("R${:.2}", info.value);
    let label_value = Label::new(Some(&value));
    label_value.add_css_class("label_value_i");

    let title = Label::new(Some(&info.title));
    title.add_css_class("title_i");

    box_center_i.append(&label_value);
    box_center_i.append(&title);

    let box_right_i = Box::new(Orientation::Vertical, 2);
    box_right_i.add_css_class("box_right_i");

    let label_status = Label::new(Some(""));
    label_status.add_css_class("label_status_i");

    if info.status {
        label_status.set_label("Paga");
        label_status.add_css_class("status_positive");
    } else {
        label_status.set_label("Devendo");
        label_status.add_css_class("status_negative");
    }

    let date = Label::new(Some(&info.date_out.to_string()));
    date.add_css_class("date_i");

    box_right_i.append(&label_status);
    box_right_i.append(&date);
    box_right_i.set_valign(gtk::Align::Center);

    box_info.append(&box_left_i);
    box_info.append(&box_center_i);
    box_info.append(&box_right_i);

    box_info
}

pub fn box_info(info: &Info) -> Box {
    let box_info = Box::new(Orientation::Horizontal, 0);
    box_info.add_css_class("box_info");

    let box_left_i = Box::new(Orientation::Vertical, 2);
    box_left_i.add_css_class("box_left_info");

    let name = Label::new(Some(&format!("{:.10}", &info.debtor)));
    name.add_css_class("name_i");
    let mut icon_path = PathBuf::from(format!("{}info_icon", var("ICON_PATH").unwrap()));
    icon_path.push(format!("{}.png", info.nature.to_lowercase()));

    let icon = Image::from_file(icon_path);
    icon.add_css_class("icon_info_box");

    if info.status {
        icon.add_css_class("positive");
    } else {
        icon.add_css_class("negative");
    }

    box_left_i.append(&name);
    box_left_i.append(&icon);
    box_left_i.set_halign(gtk::Align::Start);

    let box_center_i = Box::new(Orientation::Vertical, 2);
    box_center_i.add_css_class("box_center_info");
    box_center_i.set_valign(gtk::Align::Center);

    let value = format!("R${:.2}", info.value);
    let label_value = Label::new(Some(&value));
    label_value.add_css_class("label_value_i");

    let title = Label::new(Some(&format!(
        "{:.10}\t{}/{}",
        info.title, info.paid_installments, info.installments
    )));
    title.add_css_class("title_i");

    box_center_i.append(&label_value);
    box_center_i.append(&title);

    let box_right_i = Box::new(Orientation::Vertical, 2);
    box_right_i.add_css_class("box_right_info");

    let label_status = Label::new(Some(""));
    let button_status = Button::new();
    label_status.add_css_class("label_status_i");
    button_status.add_css_class("button_status_negative");
    button_status.add_css_class("button");

    let date = Label::new(Some(&info.date_out.to_string()));
    date.add_css_class("date_i");

    box_right_i.append(&label_status);
    box_right_i.append(&date);
    box_right_i.set_valign(gtk::Align::Center);

    if info.status {
        label_status.set_label("Paga");
        label_status.add_css_class("status_positive");
    } else {
        label_status.set_label("Devendo");
        label_status.add_css_class("status_negative");
        box_right_i.append(&button_status);
        button_status.set_label("pagar");
    }

    box_info.append(&box_left_i);
    box_info.append(&box_center_i);
    box_info.append(&box_right_i);

    box_info
}
