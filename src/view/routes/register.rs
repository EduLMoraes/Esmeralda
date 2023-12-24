#[path = "../../controller/email_valid.rs"]
mod email_valid;
use crate::controller::add_user;
use crate::structs_db::NewUser;
use crate::tokio::runtime;
use super::*;

#[component]
pub fn Register(cx: Scope) -> Element{
    let email = use_state(cx, || String::new());
    let password = use_state(cx, || String::new());
    let confirm_pass = use_state(cx, || String::new());
    let username = use_state(cx, || String::new());
    let rt = runtime::Runtime::new().unwrap();
    let nav = use_navigator(cx);
    let is_email = use_state(cx, || true);
    let is_equal = use_state(cx, || true);

    render!(
        link{
            r#rel: "stylesheet",
            href: "./src/view/styles/register.css"
        }
        div {
            id: "register",

            h1{
                i{
                    "Bem-vindo(a) à "
                    i{
                        id: "esmeralda",
                        "Esmeralda"
                    }
                }
            }
            h3{
                "Vamos criar uma conta! Já tava na hora!"
            }

            form{
                onsubmit: move |_| {
                    let user: NewUser = NewUser{
                        email: email.to_string(),
                        username: username.to_string(),
                        password: password.to_string()
                    };
                    
                    let result = rt.block_on(add_user(user, confirm_pass.to_string()));
                    
                    if result.is_ok(){
                        nav.push(Route::Home{});
                    }
                    else{
                        println!("{:?}", result.err());
                    }
                },

                input{
                    r#type: "email",
                    r#required: "true",
                    id: if **is_email { "email" } else { "input-invalid" },
                    placeholder: "Email",
                    oninput: move |input| {
                        is_email.set(email_valid::validate( &input.value ));
                        email.set(input.value.to_string())
                    }
                }   
                p { hidden: **is_email, id: "data-invalid", "Email inválido." }

                input{
                    r#type: "text",
                    r#required: "true",
                    id: "username",
                    placeholder: "Usuário",
                    oninput: move |input| {
                        username.set(input.value.to_string());
                    }
                } 

                input{
                    r#type: "password",
                    r#required: "true",
                    id: "password",
                    placeholder: "Senha",
                    oninput: move |input| {
                        password.set(input.value.to_string());
                    }
                } 

                input{
                    r#type: "password",
                    r#required: "true",
                    id: "confirm_pass",
                    placeholder: "Confirme sua senha",
                    oninput: move |input| {
                        confirm_pass.set(input.value.to_string())
                    }
                } 
                p { hidden: **is_equal, id: "data-invalid", "As senhas não são iguais." }

                button {
                    r#type: "submit",
                    id: "submit",
                    onclick: move |_| is_equal.set( password.get() == confirm_pass.get() ),
                    "Cadastrar e entrar"
                }
                p{
                    "Já possuí login? "
                    Link {
                        id: "login",
                        to: Route::Login {},
                        "Volte para tela de login."
                    }
                }
            }
        }
    )
}