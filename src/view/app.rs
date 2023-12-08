use crate::dioxus::prelude::*;

pub fn app(cx: Scope) -> Element{
    cx.render(rsx!{
        p { "Hello World!! "}
    })
}