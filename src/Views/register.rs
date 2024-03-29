use super::*;
use crate::prelude::control::add_user;
use crate::prelude::model::User::NewUser;
use crate::prelude::tokio::runtime;
use crate::prelude::Instant;
use std::path::PathBuf;
use styles::register::style_register;

/// Renders a registration form for a web application using the `yew` framework.
///
/// # Example Usage
///
/// ```rust
/// #[component]
/// pub fn Register(cx: Scope) -> Element {
///     // Function body
/// }
/// ```
///
/// # Inputs
///
/// - `cx` (Scope): The scope object provided by the `yew` framework.
///
/// # Flow
///
/// 1. Initializes several state variables using the `use_state` function provided by the `yew` framework.
/// 2. Creates a new `Runtime` object from the `tokio` crate.
/// 3. Defines a form with several input fields and a submit button.
/// 4. When the form is submitted, creates a `NewUser` object with the values entered in the input fields.
/// 5. Calls the `add_user` function with the `NewUser` object and the value of the `confirm_pass` input field.
/// 6. If the `add_user` function returns successfully, sets the `is_newly` state variable to `true`. Otherwise, prints an error message.
/// 7. Includes validation for the email and password fields, displaying error messages if the input is invalid.
/// 8. The submit button calls a function that checks if the password and confirm password fields have the same value, and updates the `is_equal` state variable accordingly.
///
/// # Outputs
///
/// - The rendered HTML element representing the registration form.
#[component]
pub fn Register(cx: Scope) -> Element {
    let path = use_shared_state::<PathBuf>(cx).unwrap();

    let email = use_state(cx, || String::new());
    let password = use_state(cx, || String::new());
    let confirm_pass = use_state(cx, || String::new());
    let username = use_state(cx, || String::new());
    let rt = runtime::Runtime::new().unwrap();
    let is_email = use_state(cx, || true);
    let is_equal = use_state(cx, || true);
    let is_newly = use_state(cx, || false);

    render!(
        style {{ style_register() }}

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

                    let now = Instant::now();
                    let result = rt.block_on(add_user(user, confirm_pass.to_string()));

                    let _ = log(path.read().clone(), &format!("[REGISTER] Register user in...[{:.3?}]\n", now.elapsed()));

                    if result.is_ok(){
                        is_newly.set(true);
                    }
                    else{
                        let _ = log(path.read().clone(), &format!("[REGISTER] {:?}\n", result.err()));
                    }
                },

                p{ hidden: !**is_newly, id: "sucess", "Cadastrado(a) com sucesso! Agora volte ao login e logue (^-^)"}

                input{
                    r#type: "email",
                    r#required: "true",
                    id: if **is_email { "email" } else { "input-invalid" },
                    placeholder: "Email",
                    oninput: move |input| {

                        let now = Instant::now();
                        is_email.set(control::is_email( &input.value ));

                        let _ = log(path.read().clone(), &format!("[REGISTER] Validate email in...[{:.3?}]\n", now.elapsed()));

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
                    "Cadastrar-se"
                }
                p{
                    "Já possuí login? "
                    dioxus_router::prelude::Link {
                        id: "login",
                        to: Route::Login {},
                        "Volte para tela de login."
                    }
                }
            }
        }
    )
}
