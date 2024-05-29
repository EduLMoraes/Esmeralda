use chrono::Datelike;

use super::*;
#[allow(deprecated)]
use crate::gtk::{
    Adjustment, AlertDialog, Calendar, CheckButton, DropDown, MessageDialog, SpinButton, Window,
};
use crate::model::Count::Count;

pub fn alert(message: &str, title: &str) {
    let window_alert = Window::new();
    window_alert.set_title(Some(title));
    window_alert.set_default_size(350, 70);

    let alert_dialog = AlertDialog::builder().message(message).build();

    alert_dialog.show(Some(&window_alert));
}

#[allow(deprecated)]
pub fn confirm(message: &str, title: &str) -> MessageDialog {
    let confirm = MessageDialog::builder()
        .buttons(gtk::ButtonsType::YesNo)
        .message_type(gtk::MessageType::Warning)
        .text(message)
        .title(title)
        .build();

    confirm
}

#[allow(deprecated)]
pub fn edit_count(title: &str, count: &Count) -> MessageDialog {
    let box_form = Box::new(Orientation::Vertical, 10);
    box_form.set_halign(gtk::Align::Center);

    let box_name = Box::new(Orientation::Vertical, 0);
    let box_title = Box::new(Orientation::Vertical, 0);
    let box_nature = Box::new(Orientation::Horizontal, 5);
    let box_date = Box::new(Orientation::Horizontal, 5);
    let box_description = Box::new(Orientation::Vertical, 0);
    let box_installments = Box::new(Orientation::Horizontal, 5);
    let box_status = Box::new(Orientation::Horizontal, 5);
    let box_value = Box::new(Orientation::Horizontal, 5);

    let name_label = Label::new(Some("*Name:"));
    let title_label = Label::new(Some("*Título:"));
    let nature_label = Label::new(Some("Natureza:"));
    let date_label = Label::new(Some("Data\ninicial:"));
    let description_label = Label::new(Some("Descrição:"));
    let installments_label = Label::new(Some("Nª de\nParcelas:"));
    let status_label = Label::new(Some("Já tá paga?"));
    let value_label = Label::new(Some("R$:\n(por parcela)"));

    let name_in = Entry::new();
    let title_in = Entry::new();
    let nature_in = DropDown::from_strings(&[
        "Casa",
        "Alimentação",
        "Transporte",
        "Saúde",
        "Lazer",
        "Receita",
        "Outros",
    ]);
    let date_in = Calendar::new();
    let date_button = Button::new();
    let description_in = Entry::new();
    let installment_in = SpinButton::new(
        Some(&Adjustment::new(0.0, 1.0, 999.0, 1.0, 0.1, 0.0)),
        1.0,
        0,
    );
    let status_in = CheckButton::new();
    let value_in = SpinButton::new(
        Some(&Adjustment::new(0.0, 0.01, 99999999.00, 0.01, 0.1, 0.0)),
        1.0,
        2,
    );

    let date_string = format!(
        "{:02}/{:02}/{:04}",
        &count.date_in.day(),
        &count.date_in.month(),
        &count.date_in.year()
    );

    name_in.set_text(&count.debtor);
    title_in.set_text(&count.title);
    description_in.set_text(&count.description);
    nature_in.set_selected(match &count.nature {
        _ => 0,
    });
    value_in.set_value(count.value as f64);
    status_in.set_active(count.status);

    name_in.set_css_classes(&["input"]);
    title_in.set_css_classes(&["input"]);
    date_in.set_css_classes(&["input", "date_input"]);
    date_button.set_css_classes(&["date_button", "input"]);
    date_in.set_visible(false);
    date_button.set_label(&date_string);

    date_in.set_size_request(10, 10);

    date_in.connect_day_selected(clone!(@weak date_button, @weak date_in => move |_| {
        let date_string = format!("{:02}/{:02}/{:04}", date_in.day(), date_in.month()+1, date_in.year());
        date_button.set_label(&date_string);
        date_in.set_visible(false);
        date_button.set_visible(true);
    }));

    date_button.connect_clicked(clone!(@weak date_button, @weak date_in => move |_| {
        date_button.set_visible(false);
        date_in.set_visible(true);
    }));

    box_name.set_halign(gtk::Align::Start);
    box_title.set_halign(gtk::Align::Start);
    box_description.set_halign(gtk::Align::Start);
    description_in.add_css_class("description_input");

    box_name.append(&name_label);
    box_name.append(&name_in);
    box_title.append(&title_label);
    box_title.append(&title_in);
    box_nature.append(&nature_label);
    box_nature.append(&nature_in);
    box_date.append(&date_label);
    box_date.append(&date_button);
    box_date.append(&date_in);
    box_description.append(&description_label);
    box_description.append(&description_in);
    box_installments.append(&installments_label);
    box_installments.append(&installment_in);
    box_status.append(&status_label);
    box_status.append(&status_in);
    box_value.append(&value_label);
    box_value.append(&value_in);

    box_form.append(&box_name);
    box_form.append(&box_title);
    box_form.append(&box_nature);
    box_form.append(&box_date);
    box_form.append(&box_description);
    box_form.append(&box_installments);
    box_form.append(&box_status);
    box_form.append(&box_value);

    let edit = MessageDialog::builder()
        .buttons(gtk::ButtonsType::OkCancel)
        .message_type(gtk::MessageType::Question)
        .child(&box_form)
        .title(title)
        .build();

    edit
}
