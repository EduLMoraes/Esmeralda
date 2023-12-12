use dioxus_router::prelude::*;
use crate::dioxus::prelude::*;

#[path = "../view/register.rs"]
mod register;
use register::Register;
#[path = "../view/login.rs"]
mod login;
use login::Login;

#[derive(Clone, Debug, PartialEq, Routable)]
pub enum Route {
   #[route("/")]
   Login {},
   #[route("/about")]
   Register {},
}