use super::*;

pub fn form_right() -> Box {
    let form = Box::new(Orientation::Vertical, 12);
    form.add_css_class("bf_right");

    let surname_label = Label::new(Some("Sobrenome:"));
    let surname_entry = Entry::new();
    let box_surname = Box::new(Orientation::Vertical, 0);
    box_surname.append(&surname_label);
    box_surname.append(&surname_entry);

    let wage_label = Label::new(Some("Qual o seu salário?"));
    let wage_entry = SpinButton::new(
        Some(&Adjustment::new(0.0, 0.00, 10000000.0, 1.0, 0.1, 0.1)),
        0.00,
        2,
    );
    let box_wage = Box::new(Orientation::Vertical, 0);
    box_wage.append(&wage_label);
    box_wage.append(&wage_entry);

    let pass_label = Label::new(Some("Confirme sua senha:*"));
    let pass_entry = Entry::new();
    let box_pass = Box::new(Orientation::Vertical, 0);
    box_pass.append(&pass_label);
    box_pass.append(&pass_entry);
    pass_entry.set_visibility(false);

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

    surname_label.set_halign(gtk::Align::Start);
    wage_label.set_halign(gtk::Align::Start);
    pass_label.set_halign(gtk::Align::Start);

    box_surname.set_halign(gtk::Align::Start);
    box_wage.set_halign(gtk::Align::Start);
    box_pass.set_halign(gtk::Align::Start);

    box_accept.add_css_class("box_accept");

    accept_link.add_css_class("link_register");
    surname_entry.add_css_class("entry_register");
    wage_entry.add_css_class("entry_register");
    pass_entry.add_css_class("entry_register");

    form.append(&box_surname);
    form.append(&box_wage);
    form.append(&box_pass);
    form.append(&box_accept);

    form
}
