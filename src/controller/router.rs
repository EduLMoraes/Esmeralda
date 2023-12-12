use dioxus_router::prelude::*;
use crate::dioxus::prelude::*;

#[path = "../view/routes/register.rs"]
mod register;
use register::Register;
#[path = "../view/routes/login.rs"]
mod login;
use login::Login;

#[path = "../view/routes/home.rs"]
mod home;
use home::Home;

#[derive(Clone, Debug, PartialEq, Routable)]
pub enum Route {
   #[route("/")]
   Login {},
   
   #[route("/register")]
   Register {},

   #[route("/home")]
   Home {},
}