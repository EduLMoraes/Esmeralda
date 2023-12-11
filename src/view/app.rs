use crate::dioxus::prelude::*;

pub fn app(cx: Scope) -> Element {
   let username: &UseState<String> = use_state(cx, || String::new());
   let password: &UseState<String> = use_state(cx, || String::new());

   cx.render(rsx! {
      div {
         style: 
            "justify-content: center; align-items: center; height: auto; text-align: center; background-color: transparent; border: 0; width: auto; display: inline-block; margin-left: 38%; height: auto; margin-top: 15%;"
            ,
         
         h1{
            i{
               "Bem-vindo(a) à Esmeralda"
            }
         }
         h3{
            "Dívidas? Nunca mais!"
         }

         form {
            style: 
               "display: inline-grid; background-color: whitesmoke; height: 150px; margin-top: 0; margin-left: 0; width: auto; border: black; padding: 20px;",

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
