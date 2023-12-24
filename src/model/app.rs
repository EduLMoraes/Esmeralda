use crate::dioxus_router::prelude::*;
use crate::dioxus::prelude::*;
use crate::env;

#[path = "../controller/router.rs"]
mod router;
use router::Route;

/// Renders a web application using the Dioxus framework.
///
/// # Arguments
///
/// * `cx` - The scope object used for rendering the web application.
///
/// # Returns
///
/// The rendered web application structure as an `Element`.
///
/// # Example
///
/// ```rust
/// let scope = Scope::new();
/// let element = app(scope);
/// ```
pub fn app(cx: Scope) -> Element {
    let version: String = String::from(env!("CARGO_PKG_VERSION"));

    cx.render(rsx! {
        link {
            r#rel: "stylesheet",
            href: "./src/view/styles/global.css"
        }

        div {
            id: "container",
            render! {
                Router::<Route> {}
            }
        }

        p {
            id: "version",
            "©Esmeralda | v{version}"
        }
    })
}
