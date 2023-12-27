use super::*;
use crate::control;
use crate::tokio::runtime;
use crate::structs_db::User;
use crate::errors::*;
use crate::Instant;
mod styles;
use styles::style_login;
use styles::style_global;

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
    let data_incompatible: &UseState<bool> = use_state(cx, || false);
    let user_not_exists: &UseState<bool> = use_state(cx, || false);

    let nav: &Navigator = use_navigator(cx);
    let rt: runtime::Runtime = runtime::Runtime::new().unwrap();


    render!(
        style {{ style_global() }}
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
                    let elapsed = now.elapsed();

                    println!("L1 -> Time to login --- [{:.3?}]", elapsed);

                    if result.is_ok(){
                        nav.push(Route::Home{});
                    }
                    else{
                        let _ = result.map_err(move |err| {
                            match err{
                                ControlError::ErrorAuthenticate(err) => { 
                                    println!("{err}"); 
                                    data_incompatible.set( true ); 
                                },
                                ControlError::ErrorExternDB(err) => {
                                    println!("{err}");
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