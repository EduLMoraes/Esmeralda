use std::borrow::BorrowMut;

use super::*;

pub fn form_left() -> Box {
    let form = Box::new(Orientation::Vertical, 12);
    form.add_css_class("bf_left");

    let name_label = Label::new(Some("Nome:*"));
    let name_entry = Entry::new();
    let box_name = Box::new(Orientation::Vertical, 0);
    box_name.append(&name_label);
    box_name.append(&name_entry);

    name_entry.connect_changed(clone!(@weak name_entry => move |input|{
        if !is_alpha(&input.text().to_string()){
            name_entry.set_css_classes(&["input_invalid", "entry_register"]);
        }else{
            let new_u = unsafe { NEWUSER.borrow_mut() };
            new_u.name = input.text().to_string();

            name_entry.set_css_classes(&["input_valid", "entry_register"]);
        }
    }));

    let login_label = Label::new(Some("Login:*"));
    let login_entry = Entry::new();
    let box_login = Box::new(Orientation::Vertical, 0);
    box_login.append(&login_label);
    box_login.append(&login_entry);

    login_entry.connect_changed(clone!(@weak login_entry => move |input|{
        let new_u = unsafe { NEWUSER.borrow_mut() };
        new_u.username = input.text().to_string();

        login_entry.set_css_classes(&["input_valid", "entry_register"]);
    }));

    let email_label = Label::new(Some("Email:*"));
    let email_entry = Entry::new();
    let box_email = Box::new(Orientation::Vertical, 0);
    box_email.append(&email_label);
    box_email.append(&email_entry);

    email_entry.connect_changed(clone!(@weak email_entry => move |input|{
        if !is_email(&input.text().to_string()){
            email_entry.set_css_classes(&["input_invalid", "entry_register"]);
        }else{
            let new_u = unsafe { NEWUSER.borrow_mut() };
            new_u.email = input.text().to_string();

            email_entry.set_css_classes(&["input_valid", "entry_register"]);
        }
    }));

    let pass_label = Label::new(Some("Senha:*"));
    let pass_entry = Entry::new();
    let box_pass = Box::new(Orientation::Vertical, 0);
    box_pass.append(&pass_label);
    box_pass.append(&pass_entry);
    pass_entry.set_visibility(false);

    pass_entry.connect_changed(clone!(@weak pass_entry => move |input|{

        if input.text().len() < 4{
            pass_entry.set_css_classes(&["input_invalid", "entry_register"]);
        }else{
            let new_u = unsafe { NEWUSER.borrow_mut() };
            new_u.password = input.text().to_string();

            pass_entry.set_css_classes(&["input_valid", "entry_register"]);
        }
    }));

    let obs = Label::new(Some("Campos com '*' são obrigatórios"));

    name_label.set_halign(gtk::Align::Start);
    email_label.set_halign(gtk::Align::Start);
    pass_label.set_halign(gtk::Align::Start);
    login_label.set_halign(gtk::Align::Start);

    box_name.set_halign(gtk::Align::Start);
    box_email.set_halign(gtk::Align::Start);
    box_pass.set_halign(gtk::Align::Start);
    box_login.set_halign(gtk::Align::Start);

    name_entry.add_css_class("entry_register");
    email_entry.add_css_class("entry_register");
    pass_entry.add_css_class("entry_register");
    login_entry.add_css_class("entry_register");

    box_name.add_css_class("box_name");
    box_email.add_css_class("box_email");
    box_pass.add_css_class("box_pass");

    obs.add_css_class("obs_register");

    form.append(&box_name);
    form.append(&box_login);
    form.append(&box_email);
    form.append(&box_pass);
    form.append(&obs);

    form
}
