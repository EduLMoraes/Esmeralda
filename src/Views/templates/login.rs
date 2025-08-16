use super::*;
use crate::prelude::model;

pub fn login_screen(stack: &Stack) -> Box {
    let screen = Box::new(Orientation::Vertical, 26);

    let box_title = Box::new(Orientation::Vertical, 0);
    let title = Label::new(Some("Esmeralda"));

    let subtitle = Label::new(Some("Dívidas? Nunca mais!"));

    box_title.append(&title);
    box_title.append(&subtitle);
    box_title.set_halign(gtk::Align::Center);

    let box_user = Box::new(Orientation::Vertical, 10);
    let box_pass = Box::new(Orientation::Vertical, 10);
    box_user.set_halign(gtk::Align::Center);
    box_pass.set_halign(gtk::Align::Center);

    let user_label = Label::new(Some("Login:"));
    let user_entry = Entry::new();
    user_entry.add_css_class("login_entry");
    box_user.append(&user_label);
    box_user.append(&user_entry);

    let pass_label = Label::new(Some("Senha:"));
    let pass_entry = Entry::new();
    pass_entry.add_css_class("login_entry");
    pass_entry.set_visibility(false);
    box_pass.append(&pass_label);
    box_pass.append(&pass_entry);

    let box_newu = Box::new(Orientation::Horizontal, 0);
    let newu_label = Label::new(Some("Não possuí uma conta?"));
    let newu_link = Button::with_label("Cadastre-se aqui!");
    box_newu.append(&newu_label);
    box_newu.append(&newu_link);

    box_pass.append(&box_newu);

    let login_button = Button::with_label("Confirmar");

    screen.set_halign(gtk::Align::Center);
    screen.set_valign(gtk::Align::Center);
    screen.set_size_request(450, 450);

    user_label.set_halign(gtk::Align::Start);
    pass_label.set_halign(gtk::Align::Start);
    user_entry.set_halign(gtk::Align::Center);
    pass_entry.set_halign(gtk::Align::Center);
    login_button.set_halign(gtk::Align::End);

    screen.append(&box_title);
    screen.append(&box_user);
    screen.append(&box_pass);
    screen.append(&login_button);

    user_entry.connect_activate(clone!(
        #[weak]
        pass_entry,
        move |_| {
            pass_entry.grab_focus();
        }
    ));

    pass_entry.connect_activate(clone!(
        #[weak]
        stack,
        #[weak]
        pass_entry,
        #[weak]
        user_entry,
        move |_| {
            let user = model::User::User {
                username: String::from(user_entry.text()),
                password: String::from(pass_entry.text()),
            };
            let run = tokio::runtime::Runtime::new().unwrap();

            if run.block_on(control::login(user)).is_ok() {
                tracing::info!("user logged!");

                let home_screen = home_screen(&stack);
                stack.add_css_class("home_window");
                stack.remove_css_class("login_window");

                stack.add_titled(&home_screen, Some("home"), "Home");
                stack.set_visible_child_name("home");

                let tmp = stack.child_by_name("login").unwrap();
                stack.remove(&tmp);

                let tmp = stack.child_by_name("register").unwrap();
                stack.remove(&tmp);
            } else {
                pass_entry.set_text("");

                tracing::error!("login failed by password or username invalid");
                alert(
                    "Senha ou usuário incorreto! Tente novamente",
                    "Falha ao logar",
                );
            }
        }
    ));

    newu_link.connect_clicked(clone!(
        #[weak]
        stack,
        move |_| {
            stack.remove_css_class("login_window");
            stack.add_css_class("register_window");
            stack.set_visible_child_name("register");
        }
    ));

    login_button.connect_clicked(clone!(
        #[weak]
        stack,
        #[weak]
        screen,
        move |_| {
            let user = model::User::User {
                username: String::from(user_entry.text()),
                password: String::from(pass_entry.text()),
            };
            let run = tokio::runtime::Runtime::new().unwrap();

            if run.block_on(control::login(user)).is_ok() {
                let home_screen = home_screen(&stack);
                stack.add_css_class("home_window");
                stack.remove_css_class("login_window");

                stack.add_titled(&home_screen, Some("home"), "Home");
                stack.set_visible_child_name("home");

                let tmp = stack.child_by_name("login").unwrap();
                stack.remove(&tmp);

                let tmp = stack.child_by_name("register").unwrap();
                stack.remove(&tmp);
            } else {
                let error = Label::new(Some("Senha ou usuário incorreto! Tente novamente"));
                pass_entry.set_text("");
                screen.append(&error);
            }
        }
    ));

    newu_link.connect_clicked(clone!(
        #[weak]
        stack,
        move |_| {
            stack.remove_css_class("login_window");
            stack.add_css_class("register_window");
            stack.set_visible_child_name("register");
        }
    ));

    screen.add_css_class("login_box");
    title.add_css_class("login_title");
    subtitle.add_css_class("login_subtitle");
    newu_link.set_css_classes(&["login_link", "link"]);
    login_button.add_css_class("login_button");
    login_button.add_css_class("button");
    box_newu.add_css_class("login_newu_box");
    box_title.add_css_class("login_title_box");
    box_user.add_css_class("login_user_box");
    box_pass.add_css_class("login_pass_box");

    screen
}
