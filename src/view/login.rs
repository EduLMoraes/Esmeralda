use super::*;
use crate::prelude::control;
use crate::prelude::errors::*;
use crate::prelude::logger::log;
use crate::prelude::structs_db::User;
use crate::prelude::tokio::runtime;
use crate::prelude::Instant;
use std::path::PathBuf;
mod styles;
use styles::style_login;

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
pub fn Login(cx: Scope) -> Element {
    let path = use_shared_state::<PathBuf>(cx).unwrap();

    let username: &UseState<String> = use_state(cx, || String::new());
    let password: &UseState<String> = use_state(cx, || String::new());
    let data_incompatible: &UseState<bool> = use_state(cx, || false);
    let user_not_exists: &UseState<bool> = use_state(cx, || false);

    let nav: &Navigator = use_navigator(cx);
    let rt: runtime::Runtime = runtime::Runtime::new().unwrap();

    render!(
        style {{ style_login() }}

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

                    let now = Instant::now();
                    let result = rt.block_on(control::login(user));

                    let _ = log(path.read().clone(), &format!("[LOGIN] Login user in...[{:.3?}]\n", now.elapsed()));

                    if result.is_ok(){
                        nav.push(Route::Home{});
                    }
                    else{
                        let _ = result.map_err(move |err| {
                            match err{
                                ControlError::ErrorAuthenticate(err) => {
                                    let _ = log(path.read().clone(), &format!("[LOGIN] {err}\n"));
                                    data_incompatible.set( true );
                                },
                                ControlError::ErrorExternDB(err) => {
                                    let _ = log(path.read().clone(), &format!("[LOGIN] {err}\n"));
                                    user_not_exists.set(true);
                                },
                                _ => { }
                            };
                        });
                    }
                },

                p{ hidden: !**user_not_exists, id: "data-invalid", "Usuário não cadastrado!" }
                p{ hidden: !**data_incompatible, id: "data-invalid", "Nome de usuário ou senha incorreto!" }

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
