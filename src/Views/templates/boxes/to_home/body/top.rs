use super::*;
use crate::chrono::Utc;
use crate::control::get_user_instance;
use chrono::Datelike;
use gtk::{DropDown, SearchEntry};

pub fn box_top(stack: &Stack) -> Box {
    let box_top = Box::new(Orientation::Horizontal, 200);

    let counts = unsafe { GLOBAL_COUNTS.get() };

    box_top.add_css_class("box_top_b");

    let title_top = Label::new(Some("Contas"));
    let select_year = match counts {
        Some(ref_counts) => {
            let tmp: Vec<String> = ref_counts.years.iter().map(|&y| y.to_string()).collect();
            let tmp: Vec<&str> = tmp.iter().map(|y| y.trim()).collect();
            DropDown::from_strings(&tmp)
        }
        None => DropDown::from_strings(&[&format!("{}", Utc::now().year())]),
    };

    select_year.connect_selected_item_notify(
        clone!(@strong select_year, @strong stack => move |_|{
            let counts = unsafe{ GLOBAL_COUNTS.get() };

            match counts{
                Some(counts) => {
                    use crate::tokio::runtime::Runtime;
                    let rnt = Runtime::new().unwrap();
                    rnt.block_on(recover(counts.years[select_year.selected() as usize])).unwrap();
                    update_list(counts);

                    let tmp = stack.child_by_name("Contas").unwrap();
                    stack.remove(&tmp);

                    stack.add_titled(&box_count(), Some("Contas"), "Contas");
                }
                None => {}
            }
        }),
    );

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

    search.connect_changed(clone!(@strong search => move |_| {
        let counts = unsafe{ GLOBAL_COUNTS.get() };

        match counts{
            Some(counts) => {
                let result = ListCount::from(
                    ListCount {
                        list: counts.search(search.text().to_string()),
                        years: vec![0]
                    }
                );

                update_list(&result);
            }
            None => {}
        }
    }));

    let button_ext = Button::with_label("Sair");
    button_ext.add_css_class("link_button");

    let username = get_user_instance().clone().unwrap();
    let username = Label::new(Some(username.username.trim()));
    username.set_halign(gtk::Align::Center);
    username.set_valign(gtk::Align::Center);
    username.set_height_request(20);

    box_top.append(&box_select);
    box_top.append(&search);
    box_top.append(&username);
    box_top
}
