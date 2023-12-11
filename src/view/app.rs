use crate::dioxus::prelude::*;

pub fn app(cx: Scope) -> Element {
   let username: &UseState<String> = use_state(cx, || String::new());
   let password: &UseState<String> = use_state(cx, || String::new());

   cx.render(rsx! {
      div {
         id: "login",
                  
         h1{
            i{
               "Bem-vindo(a) à Esmeralda"
            }
         }
         h3{
            "Dívidas? Nunca mais!"
         }

         form {
            onsubmit: move |event| {
               println!("Username: {username} has loged\nEvent: {:?}", event)
            },
            label{
               r#for: "username",
               "Usuário:"
            }
            input {
               id: "username",
               value: "{username}",
               oninput: move |evt| username.set(evt.value.clone()),
            },

            br{}
            
            label{
               r#for: "passowrd",
               "Senha:"
            }
            input {
               id: "passowrd",
               r#type: "password",
               value: "{password}",
               oninput: move |evt| password.set(evt.value.clone()),
            },

            br{}

            button {
               r#type: "submit",
               "Entrar"
            }
            
            br{}

            p{
               "Não possuí login?"
            }
            button {
               onclick: move |_| {
                     // Navegue para a página de cadastro
               },
               "Cadastre-se"
            }
         }
      }
   })
}
