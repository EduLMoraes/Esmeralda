use super::*;
use crate::prelude::chrono::Utc;

pub fn fmt_err(err: ErrorLog) -> String{
    let now = Utc::now();

    format!("\nEsmeralda: [{}] ERROR ({}) {} in {}\n -- version {}\n", now, err.code, err.title, err.file, env!("CARGO_PKG_VERSION"))
}

pub fn fmt_log(msg: String) -> String{
    let now = Utc::now();

    format!("Esmeralda v{}: [{}] {}\n", env!("CARGO_PKG_VERSION"), now, msg)
}