mod prelude;
use std::env;
use std::fs::File;
use std::io::Write;
use std::io::Read;
use prelude::{crypt::get_key, *};

fn main() {
    match var("KEYESMERALD") {
        Ok(_) => {
            println!("Variavel de ambiente já existente");
        }
        Err(_) => {
            let mut file = match File::open(".key"){
                Ok(file) => file,
                Err(_) => {
                    File::create(".key").unwrap()
                }
            };

            let mut key_env: String = String::new();
            let _ = file.read_to_string(&mut key_env);

            if key_env.is_empty(){
                key_env = get_key();
                let _ = file.write(key_env.as_bytes());
            }

            env::set_var("KEYESMERALD", key_env);
        }
    }

    dioxus_desktop::launch_cfg(
        app::app,
        config::config()
    );
}
