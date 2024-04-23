use crate::env;
use crate::File;
use libc;
use std::io::stdout;
use std::os::unix::io::AsRawFd;

pub fn get_config() {
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

    match env::var("ICON_PATH") {
        Err(_) => env::set_var("ICON_PATH", "./assets/icon/"),
        _ => {}
    }
    match env::var("IMG_PATH") {
        Err(_) => env::set_var("IMG_PATH", "./assets/img/"),
        _ => {}
    }
    match env::var("CSS_PATH") {
        Err(_) => env::set_var("CSS_PATH", "./styles/global.css"),
        _ => {}
    }
}
