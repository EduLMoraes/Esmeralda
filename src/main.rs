mod prelude;
use prelude::control::config::env_var::get_config;
use prelude::sty::load_style;
use prelude::views::esmeralda;
use prelude::*;
use std::fs::File;

fn main() {
    get_config();
    let application = Application::new(Some("myapp.Esmeralda.com"), Default::default());

    application.connect_startup(|_| load_style());
    application.connect_activate(esmeralda);
    application.run();
}
