use crate::prelude::dioxus::prelude::*;
use crate::prelude::dioxus_router::prelude::*;

#[path = "../Views/styles/mod.rs"]
mod styles;
use styles::global;

use crate::prelude::apis::Route;
use crate::prelude::env;
use crate::prelude::PathBuf;

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
#[allow(dead_code)]
pub fn app(cx: Scope) -> Element {
    let version: String = String::from(env!("CARGO_PKG_VERSION"));

    use_shared_state_provider(cx, || {
        let mut path = match std::env::consts::OS {
            "windows" => env::var("HOMEPATH").unwrap(),
            _ => env::var("HOME").unwrap(),
        };

        path.push_str("/.esmeralda/log.log");
        PathBuf::from(path)
    });

    cx.render(rsx! {

        style {{ global::style_global() }}


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
