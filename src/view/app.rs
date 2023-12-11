use dioxus_elements::link;

use crate::dioxus::prelude::*;

pub fn app(cx: Scope) -> Element {
   let username: &UseState<String> = use_state(cx, || String::new());
   let password: &UseState<String> = use_state(cx, || String::new());
   let version: String = String::from("v0.1.0");

   cx.render(rsx! {
      link{
         r#rel: "stylesheet",
         href: "./src/view/styles/login.css"
      }
      div {
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
            onsubmit: move |event| {
               println!("Username: {username} has loged\nEvent: {:?}", event)
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
               "Não possuí login?"
               a {
                  id: "register",
                  onclick: move |_| {
                        // Navegue para a página de cadastro
                  },
                  "Cadastre-se"
               }
            }
         }
      }
      p{
         id: "version",
         "{version}"
      }
   })
}
