use crate::dioxus::prelude::*;

pub fn app(cx: Scope) -> Element {
    let username: &UseState<String> = use_state(cx, || String::new());
    let password: &UseState<String> = use_state(cx, || String::new());

    cx.render(rsx! {
        div {
            style: "display: flex; justify-content: center; align-items: center; height: 100vh;",
            form {
               onsubmit: move |event| {
                  println!("Username: {username}, Password: {password}")
               },
               input {
                  value: "{username}",
                  oninput: move |evt| username.set(evt.value.clone()),
               },
               input {
                  value: "{password}",
                  oninput: move |evt| password.set(evt.value.clone()),
               },
               button {
                  r#type: "submit",
                  "Entrar"
               },
               button {
                  onclick: move |_| {
                      // Navegue para a página de cadastro
                  },
                  "Cadastrar-se"
               }
            }
        }
    })
}
