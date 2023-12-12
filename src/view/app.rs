use crate::dioxus_router::prelude::Router;
use crate::dioxus::prelude::*;
use crate::env;

#[path = "../controller/router.rs"]
mod router;
use router::Route;

pub fn app(cx: Scope) -> Element {
  
   let version: String = String::from(env!("CARGO_PKG_VERSION"));

   cx.render(rsx! {
      link{
         r#rel: "stylesheet",
         href: "./src/view/styles/global.css"
      }

      div{
         id: "container",
         render! {
            Router::<Route> { }
         }
      }

      p{
         id: "version",
         "Esmeralda | ©Eduardo Lopes de Moraes | v{version}"
      }
   })
}
