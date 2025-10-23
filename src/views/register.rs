#![allow(static_mut_refs)]

use crate::{
    controller::{
        user_controller::{add_user, login},
        view_controller::is_email,
    },
    model::user::{NewUser, User},
    views::{alerts::alert, view_home::HomeView},
};
use glib::clone;
use gtk::{
    prelude::*, Box, Button, CheckButton, Entry, Image, Label, LinkButton, Orientation, Stack,
};
use std::{
    borrow::{Borrow, BorrowMut},
    cell::RefCell,
    env::var,
};
use tokio::runtime::Runtime;

static mut NEWUSER: NewUser = NewUser {
    username: String::new(),
    email: String::new(),
    password: String::new(),
};
static mut ACCEPT: bool = false;

#[derive(Clone)]
pub struct ViewRegister {
    pub stack: RefCell<Stack>,
}

impl ViewRegister {
    pub fn new(stack: RefCell<Stack>) -> Self {
        Self { stack }
    }

    pub fn rgter_screen(&self) -> Box {
        unsafe {
            *ACCEPT.borrow_mut() = false;
        }
        self.stack
            .borrow()
            .set_css_classes(&["register_window", "window"]);
        let screen = Box::new(Orientation::Vertical, 26);

        let box_register = self.box_register();

        let return_button = Button::with_label("Voltar para login");
        let return_image = Image::from_file(format!("{}return.png", var("IMG_PATH").unwrap()));
        let box_return = Box::new(Orientation::Horizontal, 0);
        box_return.append(&return_image);
        box_return.append(&return_button);

        return_button.remove_css_class("link");

        return_image.add_css_class("return_img_register");
        return_button.add_css_class("return_register");
        box_return.add_css_class("box_return");

        screen.append(&box_return);
        screen.append(&box_register);

        return_button.connect_clicked(clone!(
            #[strong(rename_to=slf)]
            self,
            move |_| {
                slf.stack.borrow().remove_css_class("register_window");
                slf.stack.borrow().add_css_class("login_window");
                slf.stack.borrow().set_visible_child_name("login");
            },
        ));

        screen
    }

    pub fn box_register(&self) -> Box {
        let box_register = Box::new(Orientation::Vertical, 26);

        let box_title = Box::new(Orientation::Vertical, 0);
        let title = Label::new(Some("Cadastro"));
        let img = Image::from_file(format!("{}perfil-photo.png", var("IMG_PATH").unwrap()));

        box_title.append(&img);
        box_title.append(&title);
        box_title.set_halign(gtk::Align::Center);

        let box_form_main = Box::new(Orientation::Horizontal, 0);
        let box_form_left = self.form_left();
        let box_form_right = self.form_right();

        box_form_main.append(&box_form_left);
        box_form_main.append(&box_form_right);

        let login_button = Button::with_label("Confirmar");

        login_button.connect_clicked(clone!(
            #[strong(rename_to = slf)]
            self,
            move |_| {
                let new_u = unsafe { NEWUSER.borrow() };
                let accept = unsafe { ACCEPT.borrow() };

                if *accept && !new_u.is_empty() {
                    let rnt = Runtime::new().unwrap();

                    match rnt.block_on(add_user(new_u.clone())) {
                        Ok(_) => {
                            let user = User {
                                username: String::from(&new_u.username),
                                password: String::from(&new_u.password),
                            };
                            let run = tokio::runtime::Runtime::new().unwrap();
                            if run.block_on(login(user)).is_ok() {
                                let home_view = HomeView::new(slf.stack.clone());
                                slf.stack.borrow().add_titled(
                                    &home_view.home_screen(),
                                    Some("home"),
                                    "Home",
                                );
                                slf.stack.borrow().set_visible_child_name("home");

                                let tmp = slf.stack.borrow().child_by_name("login").unwrap();
                                slf.stack.borrow().remove(&tmp);

                                let tmp = slf.stack.borrow().child_by_name("register").unwrap();
                                slf.stack.borrow().remove(&tmp);
                            }
                            else {
                                alert(
                                    "Tente novamente pela tela de login!",
                                    "Falha ao realizar login",
                                );
                            }
                        }
                        Err(_) => {
                            alert(
                                "Login ou email de usuário já existente!",
                                "Falha ao cadastrar-se",
                            );
                        }
                    };
                }
                else {
                    alert(
                        "Por favor, preencha corretamente o formulário.",
                        "Entradas inválidas",
                    );
                }
            }
        ));

        box_register.set_halign(gtk::Align::Center);
        box_register.set_valign(gtk::Align::Center);
        box_register.set_size_request(450, 450);
        login_button.set_halign(gtk::Align::End);

        box_register.append(&box_title);
        box_form_right.append(&login_button);
        box_register.append(&box_form_main);

        login_button.add_css_class("button");
        title.add_css_class("register_title");
        img.add_css_class("register_image");
        box_register.add_css_class("register_box");
        box_title.add_css_class("register_title_box");
        box_form_main.add_css_class("bf_main");

        box_register
    }

    pub fn form_right(&self) -> Box {
        let form = Box::new(Orientation::Vertical, 12);
        form.add_css_class("bf_right");

        let pass_label = Label::new(Some("Senha:*"));
        let pass_entry = Entry::new();
        let box_pass = Box::new(Orientation::Vertical, 0);
        box_pass.append(&pass_label);
        box_pass.append(&pass_entry);
        pass_entry.set_visibility(false);

        pass_entry.connect_changed(clone!(
            #[weak]
            pass_entry,
            move |input| {
                if input.text().len() < 4 {
                    pass_entry.set_css_classes(&["input_invalid", "entry_register"]);
                }
                else {
                    let new_u = unsafe { NEWUSER.borrow_mut() };
                    new_u.password = input.text().to_string();

                    pass_entry.set_css_classes(&["input_valid", "entry_register"]);
                }
            }
        ));

        let pass_confirm_label = Label::new(Some("Confirme sua senha:*"));
        let pass_confirm_entry = Entry::new();
        let box_pass_confirm = Box::new(Orientation::Vertical, 0);
        box_pass_confirm.append(&pass_confirm_label);
        box_pass_confirm.append(&pass_confirm_entry);
        pass_confirm_entry.set_visibility(false);

        pass_confirm_entry.connect_changed(clone!(
            #[weak]
            pass_confirm_entry,
            move |input| {
                let new_u = unsafe { NEWUSER.borrow_mut() };

                if new_u.password != input.text() {
                    pass_confirm_entry.set_css_classes(&["input_invalid", "entry_register"]);
                }
                else {
                    pass_confirm_entry.set_css_classes(&["input_valid", "entry_register"]);
                }
            }
        ));

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

    pub fn form_left(&self) -> Box {
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
                }
                else {
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
}
