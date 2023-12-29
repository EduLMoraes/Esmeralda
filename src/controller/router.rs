use crate::prelude::dioxus::prelude::*;
use crate::prelude::dioxus_router::prelude::*;

#[path = "../view/register.rs"]
mod register;
use register::Register;
#[path = "../view/login.rs"]
mod login;
use login::Login;
#[path = "../view/home.rs"]
mod home;
use home::Home;

/// # Routable Route Enum
///
/// The `Route` enum is used for routing in a Rust application. It represents different routes that can be navigated to within the application.
///
/// ## Example Usage
///
/// ```rust
/// let route = Route::Login;
/// match route {
///     Route::Login => {
///         // handle login route
///     },
///     Route::Register => {
///         // handle register route
///     },
///     Route::Home => {
///         // handle home route
///     },
/// }
/// ```
///
/// ## Variants
///
/// - `Login`: Represents the login route. It is annotated with the route path `"/"`.
/// - `Register`: Represents the register route. It is annotated with the route path `"/register"`.
/// - `Home`: Represents the home route. It is annotated with the route path `"/static"`.
#[derive(Clone, Debug, PartialEq, Routable)]
pub enum Route {
    #[route("/")]
    Login {},

    #[route("/register")]
    Register {},

    #[route("/static")]
    Home {},
}
