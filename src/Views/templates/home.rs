use super::*;
use crate::control::{recover, recover_years, GLOBAL_COUNTS};

#[path = "./boxes/to_home/mod.rs"]
mod to_home;
use chrono::Datelike;
use to_home::*;

pub fn home_screen() -> Box {
    let screen = Box::new(Orientation::Horizontal, 0);

    let mut stack = Stack::new();

    let run = tokio::runtime::Runtime::new().unwrap();
    match run.block_on(recover_years()) {
        Ok(years) => {
            if years.len() > 0 {
                let _ = run.block_on(recover(years[0])).unwrap();
            } else {
                let _ = run
                    .block_on(recover(crate::chrono::Utc::now().year() as i16))
                    .map_err(|err| println!("{}", err));
            }
        }
        Err(_) => {}
    }

    let box_menu_left = get_box_menu_left(&stack);
    let box_body = get_box_body(&mut stack);

    screen.append(&box_menu_left);
    screen.append(&box_body);

    screen
}
