mod prelude;
use libc;
use prelude::segurance::criptography::gen_string;
use prelude::*;
use std::fs;
use std::fs::File;
use std::io::stdout;
use std::io::Read;
use std::io::Write;
use std::os::unix::io::AsRawFd;

fn main() {
    match std::env::consts::OS {
        "windows" => {
            let null_stdout = File::create("NUL").unwrap();
            let stdout_fd = stdout().as_raw_fd();
            let null_stdout_fd = null_stdout.as_raw_fd();
            unsafe {
                libc::dup2(null_stdout_fd, stdout_fd);
            }
        }
        _ => {
            let null_stdout = File::create("/dev/null").unwrap();
            let stdout_fd = stdout().as_raw_fd();
            let null_stdout_fd = null_stdout.as_raw_fd();
            unsafe {
                libc::dup2(null_stdout_fd, stdout_fd);
            }
        }
    };

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

    dioxus_desktop::launch_cfg(model::App::app, control::config::app::get_config());
}
