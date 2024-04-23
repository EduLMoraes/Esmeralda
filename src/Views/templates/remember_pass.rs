use super::*;
use crate::control::{is_alpha, restore_password};
use crate::model::User::User;

pub fn lost_pass_screen(stack: &Stack) -> Box{
    let box_screen = Box::new(Orientation::Vertical, 0);
    box_screen.add_css_class("login_box");

    let return_button = LinkButton::new("Voltar para login");
    let return_image = Image::from_file(format!("{}return.png", var("IMG_PATH").unwrap()));
    let box_return = Box::new(Orientation::Horizontal, 0);

    return_button.remove_css_class("link");
    return_button.add_css_class("return_register");
    return_image.add_css_class("return_img_register");
    box_return.add_css_class("box_return");

    box_return.append(&return_image);
    box_return.append(&return_button);
    box_screen.append(&box_return);

    return_button.connect_clicked(clone!(@weak stack => move |_| {
        stack.remove_css_class("register_window");
        stack.add_css_class("login_window");
        stack.set_visible_child_name("login");
    }));

    let login_label = Label::new(Some("Login:*"));
    let login_entry = Entry::new();
    let login_button = Button::with_label("Redefinir senha");
    let box_login = Box::new(Orientation::Vertical, 0);
    box_login.append(&login_label);
    box_login.append(&login_entry);
    box_login.append(&login_button);
    box_screen.append(&box_login);

    box_login.set_halign(gtk::Align::Center);
    box_login.set_valign(gtk::Align::Center);

    login_button.connect_clicked(clone!(@weak login_entry => move |_|{
        if !is_alpha(&login_entry.text().to_string()){
            login_entry.set_css_classes(&["input_invalid", "entry_register"]);
        }else{
            let msg_dialog = confirm("Certeza que deseja alterar a senha?", "");
            
            msg_dialog.show();

            msg_dialog.connect_response(clone!(@weak login_entry => move |msg, res| {
                use crate::gtk::ResponseType;

                if res == ResponseType::Yes{
                    let new_u = User{
                        username: login_entry.text().to_string(),
                        password: String::new()
                    };
        
                    use crate::tokio::runtime::Runtime;
                    let rnt = Runtime::new().unwrap();
        
                    let response = rnt.block_on(restore_password(new_u));
        
                    match response{
                        Ok(_) => {
                            alert("Nova senha enviada para o login cadastrado", "Senha redefinida!")},
                        Err(_) => alert(&format!("Usuário inexistente!"), "byEsmeralda")
                    }
        
                    login_entry.set_css_classes(&["input_valid", "entry_register"]);
                }

                msg.close()
            }));
        }
    }));

    box_screen
}