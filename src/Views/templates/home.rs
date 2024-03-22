use super::*;
use crate::control::recover;

#[path = "./boxes/to_home/mod.rs"]
mod to_home;
use to_home::*;

pub fn home_screen() -> Box {
    let screen = Box::new(Orientation::Horizontal, 0);

    let stack = Stack::new();

    let run = tokio::runtime::Runtime::new().unwrap();
    let _ = run.block_on(recover()).unwrap();

    let box_body = box_body(&stack);
    let box_menu_left = box_menu_left(&stack);

    screen.append(&box_menu_left);
    screen.append(&box_body);

    screen
}
