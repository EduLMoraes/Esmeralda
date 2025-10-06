use std::sync::Mutex;

use super::*;
use crate::{
    control::{recover, recover_years},
    model::List::get_counts_instance,
    prelude::model::Count::Count,
};

#[path = "./boxes/to_home/mod.rs"]
mod to_home;
use chrono::Datelike;
use control::get_user_instance;
use to_home::*;

static mut LISTBOX: OnceLock<ListBox> = OnceLock::new();
static mut BOXHOME: OnceLock<Box> = OnceLock::new();
const IS_ALERTED: Mutex<bool> = Mutex::new(false);

pub struct HomeView {}
impl HomeView {
    pub fn home_screen(screen_master: &Stack) -> Box {
        let screen = Box::new(Orientation::Horizontal, 0);

        let stack = Stack::new();

        let run = tokio::runtime::Runtime::new().unwrap();
        if let Ok(_years) = run.block_on(recover_years()) {
            let _ = run
                .block_on(recover(crate::chrono::Utc::now().year() as i16))
                .map_err(|err| println!("{err}"));
        }

        let today = chrono::Utc::now();

        if today.month()
            != get_user_instance()
                .as_ref()
                .unwrap()
                .last_login
                .parse::<u32>()
                .unwrap()
        {
            if today.month() == 1 {
                alert("Pronto para começar mais um ano? Tenho certeza que neste as coisas serão ainda melhores!", "Feliz ano novo!")
            } else {
                let month_perfomance = get_counts_instance().get_perfomance_months();
                let month_perfomance = month_perfomance[(today.month() - 2) as usize];

                if month_perfomance < 0.0 {
                    alert(&format!("Que triste :´( Seu último mês teve um rendimento negativo de R${month_perfomance:.2}"), "Abre o olho!");
                } else if month_perfomance > 0.0 {
                    alert(
                    &format!(
                        "Parabéns!!! Seu último mês teve um rendimento positivo de R${month_perfomance:.2}"
                    ),
                    "Você positivou!",
                );
                } else {
                    alert(
                        "Seu último mês teve rendimento de R$0,00",
                        "Nem pra mais, nem pra menos!",
                    );
                }
            }
        }

        #[allow(const_item_mutation)]
        if let Ok(is_alerted) = IS_ALERTED.get_mut() {
            if !*is_alerted {
                let counts = &get_counts_instance().list;
                let now = chrono::Utc::now().date_naive();
                let mut defeated_now: Vec<&Count> = Vec::new();
                let mut defeated_10days: Vec<&Count> = Vec::new();
                let mut defeated: Vec<&Count> = Vec::new();

                for count in counts {
                    if !count.status {
                        let month_pay =
                            (count.date_in.month() + count.paid_installments + 1) as i32 % 12;
                        let months_to_pay = month_pay - now.month() as i32;
                        let days_to_pay = (count.date_out.day0() as i32 + (months_to_pay * 30))
                            - now.day0() as i32;
                        let years_to_pay = count.date_out.year() - now.year();
                        let is_late = months_to_pay <= 0 && days_to_pay < 0 && years_to_pay <= 0;

                        if !is_late {
                            if days_to_pay == 0 {
                                defeated_now.push(count);
                            } else if days_to_pay <= 10 {
                                defeated_10days.push(count);
                            }
                        } else {
                            defeated.push(count);
                        }
                    }
                }

                if defeated_now.len() > 1 {
                    alert(
                        &format!(
                            "{} contas estão vencendo hoje. Atente-se ao prazo!",
                            defeated_now.len()
                        ),
                        "Vencimento de contas",
                    );
                } else if defeated_now.len() == 1 {
                    alert(
                        &format!(
                            "A conta {} esta vencendo hoje. Atente-se ao prazo!",
                            defeated_now[0].title
                        ),
                        "Vencimento de contas",
                    );
                }

                if defeated_10days.len() > 1 {
                    alert(
                        &format!(
                            "{} contas estão próximas de vencer. Atente-se ao prazo!",
                            defeated_10days.len()
                        ),
                        "Vencimento de contas",
                    );
                } else if defeated_10days.len() == 1 {
                    let count = defeated[0];
                    let month_pay =
                        (count.date_in.month() + count.paid_installments + 1) as i32 % 12;
                    let months_to_pay = month_pay - now.month() as i32;
                    let days_to_pay =
                        (count.date_out.day0() as i32 + (months_to_pay * 30)) - now.day0() as i32;

                    alert(
                        &format!(
                            "Faltam {} dias para a conta {} vencer. Atente-se ao prazo!",
                            days_to_pay, count.title
                        ),
                        "Vencimento de contas",
                    );
                }

                if defeated.len() > 1 {
                    alert(
                        &format!(
                            "{} contas venceram! Atente-se ao prazo para não pagar juros!",
                            defeated.len()
                        ),
                        "Vencimento de contas",
                    );
                } else if defeated.len() == 1 {
                    let count = defeated[0];
                    let month_pay =
                        (count.date_in.month() + count.paid_installments + 1) as i32 % 12;
                    let months_to_pay = month_pay - now.month() as i32;
                    let days_to_pay =
                        (count.date_out.day0() as i32 + (months_to_pay * 30)) - now.day0() as i32;

                    alert(
                        &format!(
                            "A conta {} venceu há {}. Atente-se ao prazo!",
                            count.title, days_to_pay
                        ),
                        "Vencimento de contas",
                    );
                }
            }
        }

        let box_menu_left = get_box_menu_left(&stack);
        let box_body = get_box_body(&stack, screen_master);

        screen.set_valign(gtk::Align::Center);
        screen.set_halign(gtk::Align::Center);

        screen.append(&box_menu_left);
        screen.append(&box_body);

        screen
    }

    pub fn reload_home(list: Option<&ListCount>, stack: std::option::Option<&Stack>) {
        let list_box = get_list_box();

        while list_box.first_child().is_some() {
            let first_child = list_box.first_child().unwrap();
            list_box.remove(&first_child);
        }

        let binding = get_counts_instance().clone();
        let counts = match list {
            Some(counts) => counts,
            None => &binding,
        };

        for count in &counts.list {
            list_box.append(&new_box_info(count));
        }

        use crate::utils::export::svg;
        svg::to_svg(counts.get_data_months(), counts.filter_debtors());

        if let Some(stack) = stack {
            if let Some(tmp) = stack.child_by_name("home") {
                stack.remove(&tmp);
                stack.add_titled(&get_home_box(stack), Some("home"), "home");
            }

            if let Some(tmp) = stack.child_by_name("payment") {
                stack.remove(&tmp);
                stack.add_titled(&get_pay_box(stack), Some("payment"), "payment");
            }

            if let Some(tmp) = stack.child_by_name("Contas") {
                stack.remove(&tmp);
                stack.add_titled(&box_count(), Some("Contas"), "Contas");
            }

            if let Some(tmp) = stack.child_by_name("Graficos") {
                stack.remove(&tmp);
                stack.add_titled(&box_graph(), Some("Graficos"), "Graficos");
            }

            if let Some(tmp) = stack.child_by_name("config") {
                stack.remove(&tmp);
                stack.add_titled(&get_config_box(stack), Some("config"), "Config");
            }

            if let Some(tmp) = stack.child_by_name("Investimentos") {
                stack.remove(&tmp);
                stack.add_titled(
                    &get_investments_box(),
                    Some("Investimentos"),
                    "Investimentos",
                );
            }
        }

        std::mem::drop(binding);
    }
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
            #[weak]
            stack,
            move |_| {
                let result = ListCount {
                    list: get_counts_instance().search(search.text().as_ref()),
                    years: vec![0],
                };

                reload_home(Some(&result), Some(&stack));
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
        box_user.set_hexpand(false);

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
                reload_home(None, Some(&stack));
                match stack.child_by_name("config") {
                    Some(_) => {
                        stack.set_visible_child_name("config");
                    }
                    None => {
                        stack.add_titled(&get_config_box(&stack), Some("config"), "Config");
                        stack.set_visible_child_name("config");
                    }
                }
            }
        ));

        let peoples = get_peoples_instance().clone();
        let mut name = "";
        if !peoples.is_empty() {
            name = &peoples[0].name;
        }

        let name = Label::new(Some(name));
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
    pub fn get_config_box(stack: &Stack) -> Box {
        let box_index = Box::new(Orientation::Vertical, 0);
        box_index.add_css_class("box_configure_user");
        box_index.append(&configs_user::get_box(stack));
        box_index
    }
    pub fn box_count() -> Box {
        let box_count = unsafe { BOXHOME.get_mut() };

        let box_count = match box_count {
            Some(box_c) => {
                let left_w = box_c.first_child().unwrap();

                box_c.remove(&left_w);
                box_c.prepend(&left());

                box_c
            }
            None => unsafe {
                BOXHOME = OnceLock::from(Box::new(Orientation::Horizontal, 0));
                let tmp = BOXHOME.get_mut().unwrap();
                tmp.append(&left());
                tmp.append(&right());

                tmp.add_css_class("box_bottom_b");

                tmp
            },
        };

        box_count.clone()
    }
    pub fn get_list_box() -> &'static mut ListBox {
        let list_box = unsafe { LISTBOX.get_mut() };

        let box_list_count = match list_box {
            Some(list_box) => list_box,
            None => unsafe {
                LISTBOX = OnceLock::from(ListBox::new());
                LISTBOX.get_mut().unwrap()
            },
        };

        box_list_count
    }
    pub fn get_box_body(stack: &Stack, stack_master: &Stack) -> Box {
        let box_body = Box::new(Orientation::Vertical, 0);

        stack.add_titled(&box_count(), Some("Contas"), "Contas");
        stack.add_titled(&box_graph(), Some("Graficos"), "Graficos");
        stack.add_titled(
            &get_investments_box(),
            Some("Investimentos"),
            "Investimentos",
        );

        box_body.append(&box_top(stack, stack_master));
        box_body.append(stack);

        box_body.add_css_class("box_body");

        box_body
    }
}
