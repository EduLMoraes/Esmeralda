use super::*;

#[path = "./config_people.rs"]
mod config_people;

pub fn get_box_config() -> Box{
    config_people::get_box()
}