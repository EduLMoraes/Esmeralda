mod prelude;
use prelude::cryptography::get_key;
use prelude::*;
use std::env;
use std::fs::File;
use std::io::Read;
use std::io::Write;

/// Checks if the "KEYESMERALD" environment variable exists. If it does, prints a message. If it doesn't, creates a file called ".key" if it doesn't exist, reads the contents of the file into a string variable called "key_env", and checks if it is empty. If it is empty, calls the "get_key" function to generate a random key, writes the key to the file, and sets the "KEYESMERALD" environment variable to the generated key. Finally, sets another environment variable called "ESMERALDSCREEN" to "login" and launches a desktop application using the "launch_cfg" function.
fn main() {
    match var("KEYESMERALD") {
        Ok(_) => {
            println!("Variavel de ambiente já existente");
        }
        Err(_) => {
            let mut file = match File::open(".key") {
                Ok(file) => file,
                Err(_) => File::create(".key").unwrap(),
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

    env::set_var("ESMERALDSCREEN", "login");

    dioxus_desktop::launch_cfg(app::app, to_app::get_config());
}
