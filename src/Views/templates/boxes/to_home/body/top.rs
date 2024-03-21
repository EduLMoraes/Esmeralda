use gtk::{DropDown, SearchEntry};

use super::*;

pub fn box_top() -> Box {
    let box_top = Box::new(Orientation::Horizontal, 200);

    box_top.add_css_class("box_top_b");

    let title_top = Label::new(Some("Contas"));
    let select_year = DropDown::from_strings(&["2023", "2024", "2022"]);

    select_year.set_halign(gtk::Align::Center);
    select_year.set_valign(gtk::Align::Center);
    select_year.set_height_request(20);

    let box_select = Box::new(Orientation::Horizontal, 0);
    box_select.add_css_class("box_select_t");
    box_select.append(&title_top);
    box_select.append(&select_year);

    let search = SearchEntry::new();
    search.set_halign(gtk::Align::Center);
    search.set_valign(gtk::Align::Center);
    search.set_height_request(20);
    search.add_css_class("search_bar_t");

    let button_ext = Button::with_label("Sair");
    button_ext.add_css_class("link_button");

    let username = Label::new(Some("Username"));
    username.set_halign(gtk::Align::Center);
    username.set_valign(gtk::Align::Center);
    username.set_height_request(20);

    box_top.append(&box_select);
    box_top.append(&search);
    box_top.append(&username);
    box_top
}
