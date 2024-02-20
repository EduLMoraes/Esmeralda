mod prelude;
use prelude::segurance::criptography::key::get_key;
use prelude::*;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::Write;

fn main() {
    match env::var("KEYESMERALD") {
        Ok(_) => {
            let mut path = match std::env::consts::OS {
                "windows" => env::var("HOMEPATH").unwrap(),
                _ => env::var("HOME").unwrap(),
            };

            path.push_str("/.esmeralda/log.log");

            let _ = log(
                path.into(),
                &format!("[MAIN] Variabel of environment already exists\n"),
            );
        }
        Err(_) => {
            let mut path = match std::env::consts::OS {
                "windows" => env::var("HOMEPATH").unwrap(),
                _ => env::var("HOME").unwrap(),
            };

            let mut file = match File::open(format!("{}/.key", &path)){
                Ok(file) => file,
                Err(_) => {
                    path.push_str("/.esmeralda/.key");
                    let file = File::create(&path).unwrap();

                    let mut perms = fs::metadata(&path).unwrap().permissions();
                    perms.set_readonly(true);
                    let _ = fs::set_permissions(&path, perms);

                    file
                }
            };

            let mut key_env: String = String::new();
            let _ = file.read_to_string(&mut key_env);

            if key_env.is_empty() {
                key_env = get_key();
                let _ = file.write(key_env.as_bytes());
            }

            env::set_var("KEYESMERALD", key_env);
        }
    }

    dioxus_desktop::launch_cfg(model::App::app, control::config::app::get_config());
}
