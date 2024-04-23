mod prelude;
use prelude::control::config::env_var::get_config;
use prelude::segurance::criptography::gen_string;
use prelude::sty::load_style;
use prelude::views::esmeralda;
use prelude::*;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::Write;

fn main() {
    get_config();

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

            let mut file = match File::open(format!("{}/.key", &path)) {
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
                key_env = gen_string(32, &[33, 126]);
                let _ = file.write(key_env.as_bytes());
            }

            env::set_var("KEYESMERALD", key_env);
        }
    }

    let application = Application::new(Some("myapp.Esmeralda.com"), Default::default());

    application.connect_startup(|_| load_style());
    application.connect_activate(esmeralda);
    application.run();
}
