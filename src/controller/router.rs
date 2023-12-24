// use dioxus_router::prelude::*;
use crate::dioxus::prelude::*;
use crate::dioxus_router::prelude::*;

#[path = "../view/register.rs"]
mod register;
use register::Register;
#[path = "../view/login.rs"]
mod login;
use login::Login;
#[path = "../view/home.rs"]
mod home;
use home::Home;

#[derive(Clone, Debug, PartialEq, Routable)]
pub enum Route {
   #[route("/")]
   Login {},
   
   #[route("/register")]
   Register {},

   #[route("/static")]
      // #[]
   Home {},
}