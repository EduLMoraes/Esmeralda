use tracing_subscriber::fmt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use crate::env;
use crate::segurance::criptography::gen_string;
use crate::File;
use std::fs;
use std::io::Read;
use std::io::Write;

/// This gen the variables of ambient if she's not exists
/// garant the functionality of system.
pub fn get_config() {
    dotenvy::dotenv().ok();
    let log_file = File::create("app.log").unwrap();

    let file_layer = fmt::layer()
        .with_writer(log_file)
        .with_ansi(false)
        .with_target(true); // Inclui o alvo do log (opcional)

    let stdout_layer = fmt::layer().with_ansi(true);

    tracing_subscriber::registry()
        .with(file_layer)
        .with(stdout_layer)
        .init();

    let path = match std::env::consts::OS {
        "windows" => env::var("HOMEPATH").unwrap(),
        _ => env::var("HOME").unwrap(),
    };

    match env::var("KEYESMERALD") {
        Ok(_) => {
            tracing::info!("Variabel of environment already exists",);
        }
        Err(_) => {
            let mut file = match File::open(format!("{path}/.key")) {
                Ok(file) => file,
                Err(_) => {
                    let file = match File::open(format!("{path}/.esmeralda/.key")) {
                        Ok(file) => file,
                        Err(_) => File::create(format!("{path}/.esmeralda/.key")).unwrap(),
                    };

                    let mut perms = fs::metadata(format!("{path}/.esmeralda/.key"))
                        .unwrap()
                        .permissions();
                    perms.set_readonly(true);
                    let _ = fs::set_permissions(format!("{path}/.esmeralda/.key"), perms);

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

    match env::var("UPDT_PATH") {
        Err(_) => env::set_var("UPDT_PATH", format!("{}/.esmeralda/", path)),
        _ => env::set_var("UPDT_PATH", format!("{}/", env::temp_dir().display())),
    }
    if env::var("YEAR_SELECTED").is_err() {
        env::set_var("YEAR_SELECTED", "0")
    }

    env::set_var("GSK_RENDERER", "cairo");

    if env::var("IMG_PATH").is_err() {
        env::set_var("IMG_PATH", format!("{}/.esmeralda/assets/img/", path))
    }
    if env::var("DB_PATH").is_err() {
        env::set_var("DB_PATH", format!("{}/.esmeralda/esmeralda.db", path))
    }
    if env::var("ICON_PATH").is_err() {
        env::set_var("ICON_PATH", format!("{}/.esmeralda/assets/icon/", path))
    }
    if env::var("CSS_PATH").is_err() {
        env::set_var("CSS_PATH", format!("{}/.esmeralda/styles/global.css", path))
    }
    if env::var("CARGO_PKG_VERSION").is_err() {
        env::set_var("CARGO_PKG_VERSION", format!("1.2.3"))
    }
    if env::var("MANAGER_PATH").is_err() {
        env::set_var("MANAGER_PATH", format!("{}/.esmeralda/manager_db", path));
    }
}
