use super::*;
use crate::chrono::Utc;
use crate::control::get_user_instance;
use chrono::Datelike;
use control::exit_user;
use gtk::{DropDown, SearchEntry};

pub fn box_top(stack: &Stack, stack_master: &Stack) -> Box {
    let box_top = Box::new(Orientation::Horizontal, 200);
    box_top.set_hexpand(true);

    box_top.add_css_class("box_top_b");

    let title_top = Label::new(Some("Contas"));
    let select_year = {
        let ref_counts = get_counts_instance();

        let tmp: Vec<String> = if !ref_counts.years.is_empty() {
            ref_counts.years.iter().map(|&y| y.to_string()).collect()
        } else {
            vec![format!("{}", Utc::now().year())]
        };
        let tmp: Vec<&str> = tmp.iter().map(|y| y.trim()).collect();
        DropDown::from_strings(&tmp)
    };
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

    let username = get_user_instance().clone().unwrap();
    let username = Label::new(Some(username.username.trim()));
    username.set_halign(gtk::Align::Center);
    username.set_valign(gtk::Align::Center);
    username.set_height_request(20);

    box_top.append(&box_select);
    box_top.append(&search);
    box_top.append(&username);
    box_top.append(&button_ext);
    box_top
}
