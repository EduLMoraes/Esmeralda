use prelude::export::*;
use prelude::structs::InterfaceInfo;
use prelude::{crypt::get_key, *};
mod prelude;
use std::env;
use std::fs::File;
use std::io::Write;
use std::io::Read;

fn main() {

    let data = InterfaceInfo::test();

    tokio::runtime::Runtime::new().unwrap().block_on(export_pdf("./teste.pdf", data));

    // match var("KEYESMERALD") {
    //     Ok(_) => {
    //         println!("Variavel de ambiente já existente");
    //     }
    //     Err(_) => {
    //         let mut file = match File::open(".key"){
    //             Ok(file) => file,
    //             Err(_) => {
    //                 File::create(".key").unwrap()
    //             }
    //         };

    //         let mut key_env: String = String::new();
    //         let _ = file.read_to_string(&mut key_env);

    //         if key_env.is_empty(){
    //             key_env = get_key();
    //             let _ = file.write(key_env.as_bytes());
    //         }

    //         env::set_var("KEYESMERALD", key_env);
    //     }
    // }

    // env::set_var("ESMERALDSCREEN", "login");

    // dioxus_desktop::launch_cfg(
    //     app::app,
    //     config::config()
    // );
}
