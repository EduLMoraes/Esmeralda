use crate::{
    controller::config::env_var::get_config,
    views::{app::esmeralda, styles::load_style},
};
use gtk::Application;

mod apis;
mod controller;
mod model;
mod segurance;
mod utils;
mod views;

fn main() {
    get_config();
    let application = Application::new(Some("myapp.Esmeralda.com"), Default::default());

    application.connect_startup(|_| load_style());
    application.connect_activate(esmeralda);
    application.run();
}
