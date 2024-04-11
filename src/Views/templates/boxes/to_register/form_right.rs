use std::borrow::BorrowMut;

use super::*;

pub fn form_right() -> Box {
    let form = Box::new(Orientation::Vertical, 12);
    form.add_css_class("bf_right");

    let wage_label = Label::new(Some("Qual o seu salário?"));
    let wage_entry = SpinButton::new(
        Some(&Adjustment::new(0.0, 0.00, 10000000.0, 1.0, 0.1, 0.1)),
        0.00,
        2,
    );

    let box_wage = Box::new(Orientation::Vertical, 0);
    box_wage.append(&wage_label);
    box_wage.append(&wage_entry);

    wage_entry.connect_changed(clone!(@weak wage_entry => move |input|{
        let new_u = unsafe { NEWUSER.borrow_mut() };

        wage_entry.set_css_classes(&["input_valid", "entry_register"]);
        new_u.wage = input.value() as f32;
    }));

    let pass_label = Label::new(Some("Confirme sua senha:*"));
    let pass_entry = Entry::new();
    let box_pass = Box::new(Orientation::Vertical, 0);
    box_pass.append(&pass_label);
    box_pass.append(&pass_entry);
    pass_entry.set_visibility(false);

    pass_entry.connect_changed(clone!(@weak pass_entry => move |input|{
        let new_u = unsafe { NEWUSER.borrow_mut() };

        if new_u.password != input.text(){
            pass_entry.set_css_classes(&["input_invalid", "entry_register"]);
        }else{
            pass_entry.set_css_classes(&["input_valid", "entry_register"]);
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

    wage_label.set_halign(gtk::Align::Start);
    pass_label.set_halign(gtk::Align::Start);

    box_wage.set_halign(gtk::Align::Start);
    box_pass.set_halign(gtk::Align::Start);

    box_accept.add_css_class("box_accept");

    accept_link.add_css_class("link_register");
    wage_entry.add_css_class("entry_register");
    pass_entry.add_css_class("entry_register");

    form.append(&box_wage);
    form.append(&box_pass);
    form.append(&box_accept);

    form
}
