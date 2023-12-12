use crate::dioxus::prelude::*;

#[component]
pub fn Register(cx: Scope) -> Element{
    render!(
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
                    println!("Criada")
                },   
            }
            
        }
    )
}