use super::*;
use crate::prelude::model::User::User;
use crate::prelude::tokio::runtime;
use crate::prelude::Instant;
use std::path::PathBuf;
use styles::login::style_login;

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
pub fn Restore(cx: Scope) -> Element {
    let path = use_shared_state::<PathBuf>(cx).unwrap();

    let username: &UseState<String> = use_state(cx, || String::new());
    let user_not_exists: &UseState<bool> = use_state(cx, || false);

    let nav: &Navigator = dioxus_router::prelude::use_navigator(cx);
    let rt: runtime::Runtime = runtime::Runtime::new().unwrap();

    render!(
        style {{ style_login() }}

        div{
            id: "login",

            h3{
                "Bora recuperar essa senha?"
            }

            form {
                onsubmit: move |_| {
                    let user: User = User{
                        username: username.to_string(),
                        password: String::from("")
                    };

                    let now = Instant::now();

                    let result = rt.block_on(control::restore_password(user));

                    if result.is_ok(){
                        nav.push(Route::Login{});
                    }
                    else{
                        let _ = result.map_err(move |err| {
                            match err{
                                ControlError::UserNotExists(err) => {
                                    let _ = log(path.read().clone(), &format!("[RESTORE] {err}\n"));
                                },
                                ControlError::ErrorExternDB(err) => {
                                    let _ = log(path.read().clone(), &format!("[RESTORE] {err}\n"));
                                    user_not_exists.set(true);
                                },
                                _ => { }
                            };
                        });
                    }

                    let _ = log(path.read().clone(), &format!("[RESTORE] Restore password in...[{:.3?}]\n", now.elapsed()));
                },

                p{ hidden: !**user_not_exists, id: "data-invalid", "Usuário não cadastrado!" }

                input {
                    id: "username",
                    placeholder: "username",
                    value: "{username}",
                    oninput: move |evt| username.set(evt.value.clone()),
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
                    dioxus_router::prelude::Link {
                        id: "register",
                        to: Route::Register {},
                        "Cadastre-se"
                    }
                }
                p{
                    "Não esqueceu a senha? "
                    dioxus_router::prelude::Link {
                        id: "register",
                        to: Route::Login {},
                        "Voltar para login"
                    }
                }

            }
        }
    )
}
