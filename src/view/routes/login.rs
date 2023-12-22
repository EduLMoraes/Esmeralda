use crate::prelude::controller;
use crate::tokio::runtime;
use crate::structs_db::User;
use super::*;

#[component]
pub fn Login(cx: Scope) -> Element{
    let username: &UseState<String> = use_state(cx, || String::new());
    let password: &UseState<String> = use_state(cx, || String::new());

    let nav = use_navigator(cx);
    let rt = runtime::Runtime::new().unwrap();

    render!(
        link{
            r#rel: "stylesheet",
            href: "./src/view/styles/login.css"
        }
        div{
            id: "login",

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
                "Dívidas? Nunca mais!"
            }

            form {
                onsubmit: move |_| {
                    let user: User = User{
                        username: username.to_string(),
                        password: password.to_string()
                    };
                    
                    let result = rt.block_on(controller::login(user));
                    
                    if result.is_ok(){
                        nav.push(Route::Home{});
                    }
                    else{
                        println!("{:?}", result.err());
                    }
                },
                
                input {
                    id: "username",
                    placeholder: "Username",
                    value: "{username}",
                    oninput: move |evt| username.set(evt.value.clone()),
                },

                br{}
                
                input {
                    id: "passowrd",
                    r#type: "password",
                    placeholder: "Senha",
                    value: "{password}",
                    oninput: move |evt| password.set(evt.value.clone()),
                },

                br{}

                button {
                    r#type: "submit",
                    id: "submit",
                    "Entrar"
                }
                
                br{}

                p{
                    "Não possuí login? "
                    Link {
                        id: "register",
                        to: Route::Register {},
                        "Cadastre-se"
                    }
                }
                
            }
        }
    )
}