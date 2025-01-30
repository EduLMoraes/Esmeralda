use std::env;

use super::*;
// use crate::control::get_user_instance;
use crate::chrono::Utc;
use crate::model::List::get_counts_instance;
use crate::prelude::control::get_peoples_instance;
use chrono::Datelike;
use control::exit_user;
use glib::clone;
use gtk::{DropDown, SearchEntry};

pub fn box_top(stack: &Stack, stack_master: &Stack) -> Box {
    let box_top = Box::new(Orientation::Horizontal, 100);
    box_top.set_hexpand(true);

    box_top.add_css_class("box_top_b");

    let title_top = Label::new(Some("Contas"));
    let mut actual_year: u32 = 0;
    let select_year = {
        let ref_counts = get_counts_instance();

        for i in 0..ref_counts.years.len() {
            if ref_counts.years[i] == chrono::Utc::now().year() as i16 {
                actual_year = i as u32;
                break;
            }
        }

        let tmp: Vec<String> = if !ref_counts.years.is_empty() {
            ref_counts.years.iter().map(|&y| y.to_string()).collect()
        } else {
            vec![format!("{}", Utc::now().year())]
        };
        let tmp: Vec<&str> = tmp.iter().map(|y| y.trim()).collect();
        DropDown::from_strings(&tmp)
    };

    select_year.set_selected(actual_year);
    select_year.add_css_class("dropdown_select_year");

    select_year.connect_selected_item_notify(clone!(
        #[weak]
        stack,
        move |select_year| {
            let counts = get_counts_instance().years.clone();

            use crate::tokio::runtime::Runtime;
            let rnt = Runtime::new().unwrap();
            rnt.block_on(recover(counts[select_year.selected() as usize]))
                .unwrap();

            reload_home(None, Some(&stack));
        }
    ));

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

    search.connect_changed(clone!(
        #[weak]
        search,
        move |_| {
            let result = ListCount {
                list: get_counts_instance().search(&search.text().to_string()),
                years: vec![0],
            };

            reload_home(Some(&result), None);
        }
    ));

    let button_ext = Button::with_label("Sair");
    button_ext.set_halign(gtk::Align::Center);
    button_ext.set_valign(gtk::Align::Center);
    button_ext.add_css_class("link_button");

    button_ext.connect_clicked(clone!(
        #[weak]
        stack_master,
        move |_| {
            let login_screen = login_screen(&stack_master);
            let register_screen = rgter_screen(&stack_master);

            stack_master.add_titled(&login_screen, Some("login"), "Login");
            stack_master.add_titled(&register_screen, Some("register"), "Register");

            stack_master.remove_css_class("home_window");
            stack_master.add_css_class("login_window");
            stack_master.set_visible_child_name("login");

            let tmp = stack_master.child_by_name("home").unwrap();
            stack_master.remove(&tmp);

            exit_user();
        }
    ));

    let box_user = Box::new(Orientation::Horizontal, 1);

    let icon_config =
        Image::from_file(format!("{}perfil-photo.png", env::var("IMG_PATH").unwrap()));
    icon_config.set_hexpand(true);
    icon_config.set_vexpand(true);
    let button_config = Button::builder()
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .css_classes(["button_to_config"])
        .child(&icon_config)
        .build();

    button_config.connect_clicked(clone!(
        #[weak]
        stack,
        move |_| {
            match stack.child_by_name("config") {
                Some(_) => {
                    stack.set_visible_child_name("config");
                }
                None => {
                    stack.add_titled(&get_config_box(), Some("config"), "Config");
                    stack.set_visible_child_name("config");
                }
            }
        }
    ));
    let name = get_peoples_instance().clone();
    let name = Label::new(Some(&name[0].name));
    name.set_halign(gtk::Align::Center);
    name.set_valign(gtk::Align::Center);
    name.set_height_request(20);

    box_user.append(&name);
    box_user.append(&button_config);

    box_top.append(&box_select);
    box_top.append(&search);
    box_top.append(&box_user);
    box_top.append(&button_ext);
    box_top
}
