use super::*;
use crate::control;
use crate::tokio::runtime;
use crate::structs_db::User;
use crate::errors::*;

#[component]
/// Renders a login form.
///
/// # Arguments
///
/// * `cx` - The scope object used to create the login form.
///
/// # Example
///
/// ```rust
/// let cx: Scope = ...; // create a scope object
/// let login_form: Element = Login(cx); // render the login form
/// ```
pub fn Login(cx: Scope) -> Element{
    let username: &UseState<String> = use_state(cx, || String::new());
    let password: &UseState<String> = use_state(cx, || String::new());
    let login_failed: &UseState<bool> = use_state(cx, || false);

    let nav: &Navigator = use_navigator(cx);
    let rt: runtime::Runtime = runtime::Runtime::new().unwrap();


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
                    
                    let result = rt.block_on(control::login(user));
                    
                    if result.is_ok(){
                        nav.push(Route::Home{});
                    }
                    else{
                        let _ = result.map_err(move |err| {
                            match err{
                                ControlError::ErrorAuthenticate(_) => { login_failed.set( true ) },
                                _ => {}
                            };
                        });
                    }
                },
                
                p{ hidden: !**login_failed, id: "data-invalid", "Nome de usuário ou senha incorreto!" }

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