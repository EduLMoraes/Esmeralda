mod prelude;
use prelude::cryptography::get_key;
use prelude::logger::log;
use prelude::*;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::Write;

fn main() {
    match var("KEYESMERALD") {
        Ok(_) => {
            let mut path = match std::env::consts::OS {
                "windows" => env::var("HOMEPATH").unwrap(),
                _ => env::var("HOME").unwrap(),
            };
            path.push_str("/esmeralda/log.log");

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

            path.push_str("/.key");

            let mut file = match File::open(&path) {
                Ok(file) => file,
                Err(_) => File::create(&path).unwrap(),
            };

            let mut perms = fs::metadata(&path).unwrap().permissions();
            perms.set_readonly(true);
            let _ = fs::set_permissions(&path, perms);

            let mut key_env: String = String::new();
            let _ = file.read_to_string(&mut key_env);

            if key_env.is_empty() {
                key_env = get_key();
                let _ = file.write(key_env.as_bytes());
            }

            env::set_var("KEYESMERALD", key_env);
        }
    }

    dioxus_desktop::launch_cfg(app::app, to_app::get_config());
}
