use super::*;

pub fn get_config_box() -> Box {
    let box_index = Box::new(Orientation::Vertical, 0);
    box_index.add_css_class("box_configure_user");
    box_index.append(&configs_user::get_box());
    box_index
}
