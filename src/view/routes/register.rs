use crate::controller::add_user;
use crate::structs::NewUser;
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
                    id: "email",
                    placeholder: "Email",
                    oninput: move |input| email.set(input.value.to_string())
                }   
                input{
                    r#type: "text",
                    id: "username",
                    placeholder: "Usuário",
                    oninput: move |input| username.set(input.value.to_string())
                } 
                input{
                    r#type: "password",
                    id: "password",
                    placeholder: "Senha",
                    oninput: move |input| password.set(input.value.to_string())
                } 
                input{
                    r#type: "password",
                    id: "confirm_pass",
                    placeholder: "Confirme sua senha",
                    oninput: move |input| confirm_pass.set(input.value.to_string())
                } 

                button {
                    r#type: "submit",
                    id: "submit",
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