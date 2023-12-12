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
                "Dívidas? Nunca mais!"
            }

            p{
                "registrador"
            }
            
        }
    )
}