use std::fs::File;
use std::{path::Path, io::Write};
use std::env;

use crate::prelude::export::mkdir;
use crate::prelude::errors::ErrorLog;
use crate::prelude::tokio;

mod format;
use format::{fmt_err, fmt_log};

pub fn log(action: Option<String>, err: Option<ErrorLog>) -> String{
    let msg = match err{
        Some(err) => fmt_err(err),
        None => match action{
            Some(action) => fmt_log(action),
            None => log(None, Some(ErrorLog { title: "Input log inválid.", code: 1000, file: "whrite.rs" }))
        },
    };

    let path = env::var("HOME").unwrap();
    let path = Path::new(path.trim());
    let path = path.join("/esmeralda/logs/log.txt");

    let rnt = tokio::runtime::Runtime::new().unwrap();



    let file = File::open(path.as_path().to_str().unwrap());

   

    match file{
        Ok(mut file) => {
            let mut log_file = String::new();
            log_file.push_str(
                &msg
            );

            let _ = file.write_all(log_file.as_bytes()).map_err(|e|  
                log(None, Some(ErrorLog { title: &e.to_string(), code: 1000, file: "whrite.rs" }))
            );

            println!("{}", msg);
            msg
        },
        Err(e) => {
            let (_, _) = rnt.block_on( mkdir(path.as_path().to_str().unwrap()) ).unwrap();

            log(None, Some(ErrorLog { title: &e.to_string(), code: 1000, file: "whrite.rs" }))
        },
    }
}