mod prelude;
use std::env;

use prelude::{crypt::get_key, *};

fn main() {
    match var("KEYESMERALD") {
        Ok(_) => {
            println!("Variavel de ambiente já existente");
        }
        Err(_) => {
            let key_env = get_key();

            env::set_var("KEYESMERALD", key_env);
        }
    }

    crypt::crpt("hello World".to_string());
    crypt::crpt("hello World".to_string());
    // dioxus_desktop::launch_cfg(
    //     app::app,
    //     config::config(),
    // );
}
