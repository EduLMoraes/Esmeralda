use super::*;
use gtk::CheckButton;
use std::borrow::BorrowMut;

pub fn form_right() -> Box {
    let form = Box::new(Orientation::Vertical, 12);
    form.add_css_class("bf_right");

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

    let pass_confirm_label = Label::new(Some("Confirme sua senha:*"));
    let pass_confirm_entry = Entry::new();
    let box_pass_confirm = Box::new(Orientation::Vertical, 0);
    box_pass_confirm.append(&pass_confirm_label);
    box_pass_confirm.append(&pass_confirm_entry);
    pass_confirm_entry.set_visibility(false);

    pass_confirm_entry.connect_changed(clone!(@weak pass_confirm_entry => move |input|{
        let new_u = unsafe { NEWUSER.borrow_mut() };

        if new_u.password != input.text(){
            pass_confirm_entry.set_css_classes(&["input_invalid", "entry_register"]);
        }else{
            pass_confirm_entry.set_css_classes(&["input_valid", "entry_register"]);
        }
    }));

    let accept_label = Label::new(Some("Li e aceito as"));
    let accept_link = LinkButton::with_label(
        "https://github.com/EduLMoraes/Esmeralda",
        "políticas de privacidade",
    );
    let accept_check = CheckButton::new();
    let box_accept = Box::new(Orientation::Horizontal, 0);
    box_accept.append(&accept_check);
    box_accept.append(&accept_label);
    box_accept.append(&accept_link);

    accept_check.connect_toggled(|input| unsafe { ACCEPT = input.is_active() });

    pass_label.set_halign(gtk::Align::Start);
    pass_confirm_label.set_halign(gtk::Align::Start);

    box_pass.set_halign(gtk::Align::Start);
    box_pass_confirm.set_halign(gtk::Align::Start);

    box_accept.add_css_class("box_accept");
    box_pass.add_css_class("box_pass");
    box_pass_confirm.add_css_class("box_pass");

    accept_link.add_css_class("link_register");
    pass_entry.add_css_class("entry_register");
    pass_confirm_entry.add_css_class("entry_register");

    form.append(&box_pass);
    form.append(&box_pass_confirm);
    form.append(&box_accept);

    form
}
