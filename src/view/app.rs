use crate::dioxus::prelude::*;
use dioxus_html::style;

pub fn app(cx: Scope) -> Element{
    cx.render(rsx!(div{
        display: "flex",
        div{
            display: "flex",
            "ola"
        }
        "hi"
    }))
}