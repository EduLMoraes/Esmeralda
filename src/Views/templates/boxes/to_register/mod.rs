use std::borrow::Borrow;

use super::*;
use crate::control::add_user;
use crate::control::is_alpha;
use crate::control::is_email;
use crate::env::var;
use crate::model::User::NewUser;
use crate::model::User::User;

static mut NEWUSER: NewUser = NewUser {
    username: String::new(),
    email: String::new(),
    password: String::new(),
    name: String::new(),
    wage: 0.0,
};
static mut ACCEPT: bool = false;

mod form_left;
mod form_right;
pub use form_left::*;
pub use form_right::*;

pub fn box_register(stack: &Stack) -> Box {
    let box_register = Box::new(Orientation::Vertical, 26);

    let box_title = Box::new(Orientation::Vertical, 0);
    let title = Label::new(Some("Cadastro"));
    let img = Image::from_file(format!("{}perfil-photo.png", var("IMG_PATH").unwrap()));

    box_title.append(&img);
    box_title.append(&title);
    box_title.set_halign(gtk::Align::Center);

    let box_form_main = Box::new(Orientation::Horizontal, 0);
    let box_form_left = form_left();
    let box_form_right = form_right();

    box_form_main.append(&box_form_left);
    box_form_main.append(&box_form_right);

    let login_button = Button::with_label("Confirmar");

    login_button.connect_clicked(clone!(@weak stack, @weak box_register => move |_|{
        let new_u = unsafe { NEWUSER.borrow() };
        let accept = unsafe { ACCEPT.borrow() };

        if *accept && !new_u.is_empty(){
            use crate::tokio::runtime::Runtime;
            let rnt = Runtime::new().unwrap();

            match rnt.block_on(add_user(new_u.clone())){
                Ok(_) => {
                    let user = User{
                        username: String::from(&new_u.username),
                        password: String::from(&new_u.password)
                    };
                    let run = tokio::runtime::Runtime::new().unwrap();

                    if run.block_on(control::login(user)).is_ok(){
                        let home_screen = home_screen();
                        stack.add_css_class("home_window");
                        stack.remove_css_class("register_window");

                        stack.add_titled(&home_screen, Some("home"), "Home");
                        stack.set_visible_child_name("home");

                        let tmp = stack.child_by_name("login").unwrap();
                        stack.remove(&tmp);

                        let tmp = stack.child_by_name("register").unwrap();
                        stack.remove(&tmp);

                    }else{
                        alert("Tente novamente pela tela de login!", "Falha ao realizar login");
                    }
                },
                Err(_) => {
                    alert("Login ou email de usuário já existente!", "Falha ao cadastrar-se");
                }
            };
        } else{
            alert("Por favor, preencha corretamente o formulário.", "Entradas inválidas");
        }

    }));

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
