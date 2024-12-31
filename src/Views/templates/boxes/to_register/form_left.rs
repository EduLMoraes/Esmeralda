use std::borrow::BorrowMut;

use super::*;

pub fn form_left() -> Box {
    let form = Box::new(Orientation::Vertical, 12);
    form.add_css_class("bf_left");

    let login_label = Label::new(Some("Login:*"));
    let login_entry = Entry::new();
    let box_login = Box::new(Orientation::Vertical, 0);
    box_login.append(&login_label);
    box_login.append(&login_entry);

    login_entry.connect_changed(clone!(
        #[weak]
        login_entry,
        move |input| {
            let new_u = unsafe { NEWUSER.borrow_mut() };
            new_u.username = input.text().to_string();

            login_entry.set_css_classes(&["input_valid", "entry_register"]);
        }
    ));

    let email_label = Label::new(Some("Email:*"));
    let email_entry = Entry::new();
    let box_email = Box::new(Orientation::Vertical, 0);
    box_email.append(&email_label);
    box_email.append(&email_entry);

    email_entry.connect_changed(clone!(
        #[weak]
        email_entry,
        move |input| {
            if !is_email(input.text().as_ref()) {
                email_entry.set_css_classes(&["input_invalid", "entry_register"]);
            } else {
                let new_u = unsafe { NEWUSER.borrow_mut() };
                new_u.email = input.text().to_string();

                email_entry.set_css_classes(&["input_valid", "entry_register"]);
            }
        }
    ));

    let obs = Label::new(Some("Campos com '*' são obrigatórios"));

    email_label.set_halign(gtk::Align::Start);
    login_label.set_halign(gtk::Align::Start);

    box_email.set_halign(gtk::Align::Start);
    box_login.set_halign(gtk::Align::Start);

    email_entry.add_css_class("entry_register");
    login_entry.add_css_class("entry_register");

    box_email.add_css_class("box_email");

    obs.add_css_class("obs_register");

    form.append(&box_login);
    form.append(&box_email);
    form.append(&obs);

    form
}
