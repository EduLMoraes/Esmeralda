use crate::{
    controller::{exit_user, get_peoples_instance, get_user_instance, recover, recover_years},
    model::{Count::Count, List::get_counts_instance},
    views::{
        alerts::alert,
        login::ViewLogin,
        register::ViewRegister,
        view_home::{box_count::BoxCount, box_left_menu::LeftMenu, box_user::BoxUser},
    },
};
use chrono::Utc;
use glib::clone;
use gtk::{
    prelude::WidgetExt, Align, Box, Button, DropDown, Image, Label, ListBox, Orientation,
    SearchEntry, Stack,
};
use std::{
    env,
    sync::{Mutex, OnceLock},
};
use tokio::runtime::Runtime;

mod box_count;
mod box_investment;
mod box_left_menu;
mod box_plot;
mod box_user;

static mut LISTBOX: OnceLock<ListBox> = OnceLock::new();
static mut BOXHOME: OnceLock<Box> = OnceLock::new();
const IS_ALERTED: Mutex<bool> = Mutex::new(false);

pub struct HomeView<'a, T> {
    pub global_stack: &'a Stack,
    pub home_stack: Stack,
    pub current_page: Option<T>,
    pub has_alerted: bool,
}

impl<'a, T> HomeView<'a, T> {
    pub fn new(global_stack: &Stack) -> Self {
        let view_user = Box::new(Orientation::Vertical, 0);
        view_user.add_css_class("box_configure_user");
        let box_user = BoxUser::new(global_stack);
        view_user.append(&box_user.get_box());

        let view_count = Box::new(Orientation::Vertical, 0);

        Self {
            global_stack,
            home_stack: Stack::new(),
            current_page: None,
            has_alerted: false,
        }
    }

    pub fn load_page(&self, page: T)
    where
        T: WidgetExt,
    {
        let current_child = self.home_stack.first_child();
        if let Some(current_child) = current_child {
            self.home_stack.remove(current_child);
        }
        self.current_page = Some(page);
        self.home_stack
            .add_titled(page, Some(page.TITLE), page.TITLE);
        self.home_stack.set_visible_child_name(page.TITLE);
    }

    pub fn home_screen(&mut self) -> Box {
        self.global_stack
            .set_css_classes(&["home_window", "window"]);

        let screen = Box::new(Orientation::Horizontal, 0);

        let run = tokio::runtime::Runtime::new().unwrap();

        if let Ok(_years) = run.block_on(recover_years()) {
            let _ = run
                .block_on(recover(Utc::now().year() as i16))
                .map_err(|err| println!("{err}"));
        }

        self.perfomance_resume();
        self.alert_dead_line();

        screen.set_valign(Align::Center);
        screen.set_halign(Align::Center);

        screen.append(&LeftMenu::new(self));
        screen.append(&self.get_box_body());

        screen
    }

    pub fn reload_home(&self) {
        self.load_page(self.current_page);
    }

    pub fn box_top(&self) -> Box {
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
            }
            else {
                vec![format!("{}", Utc::now().year())]
            };
            let tmp: Vec<&str> = tmp.iter().map(|y| y.trim()).collect();
            DropDown::from_strings(&tmp)
        };

        select_year.set_selected(actual_year);
        select_year.add_css_class("dropdown_select_year");
        select_year.connect_selected_item_notify(clone!(move |select_year| {
            let counts = get_counts_instance().years.clone();

            let rnt = Runtime::new().unwrap();
            rnt.block_on(recover(counts[select_year.selected() as usize]))
                .unwrap();

            self.reload_home();
        }));

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

        search.connect_changed(move |_| {
            self.reload_home();
        });

        let button_ext = Button::with_label("Sair");
        button_ext.set_halign(gtk::Align::Center);
        button_ext.set_valign(gtk::Align::Center);
        button_ext.add_css_class("link_button");

        button_ext.connect_clicked(clone!(move |_| {
            let login_screen = ViewLogin::new(&self.global_stack);
            let register_screen = ViewRegister::new(&self.global_stack);

            self.global_stack
                .add_titled(&login_screen.login_screen(), Some("login"), "Login");
            self.global_stack.add_titled(
                &register_screen.rgter_screen(),
                Some("register"),
                "Register",
            );

            self.global_stack.set_visible_child_name("login");

            let tmp = self.global_stack.child_by_name("home").unwrap();
            self.global_stack.remove(&tmp);

            exit_user();
        }));

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

        button_config.connect_clicked(move |_| {
            self.load_page(BoxUser::new(&self.home_stack));
        });

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

    pub fn get_box_body(&self) -> Box {
        let box_body = Box::new(Orientation::Vertical, 0);
        self.load_page(BoxCount::new());

        box_body.append(self.box_top());
        box_body.append(self.home_stack);
        box_body.add_css_class("box_body");

        box_body
    }

    pub fn perfomance_resume(&self) {
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
            }
            else {
                let month_perfomance = get_counts_instance().get_perfomance_months();
                let month_perfomance = month_perfomance[(today.month() - 2) as usize];

                if month_perfomance < 0.0 {
                    alert(&format!("Que triste :´( Seu último mês teve um rendimento negativo de R${month_perfomance:.2}"), "Abre o olho!");
                }
                else if month_perfomance > 0.0 {
                    alert(
                    &format!(
                        "Parabéns!!! Seu último mês teve um rendimento positivo de R${month_perfomance:.2}"
                    ),
                    "Você positivou!",
                );
                }
                else {
                    alert(
                        "Seu último mês teve rendimento de R$0,00",
                        "Nem pra mais, nem pra menos!",
                    );
                }
            }
        }
    }

    pub fn alert_dead_line(&mut self) {
        if !self.has_alerted {
            let counts = &get_counts_instance().list;
            let now = chrono::Utc::now().date_naive();
            let mut defeated_now: Vec<&Count> = Vec::new();
            let mut defeated_10days: Vec<&Count> = Vec::new();
            let mut defeated: Vec<&Count> = Vec::new();

            for count in counts {
                if !count.status {
                    let month_pay =
                        (count.date_in.month0() + count.paid_installments + 1) as i32 % 12;
                    let months_to_pay = month_pay - now.month0() as i32;
                    let days_to_pay =
                        (count.date_out.day0() as i32 + (months_to_pay * 30)) - now.day0() as i32;
                    let years_to_pay = count.date_out.year() - now.year0();
                    let is_late = months_to_pay <= 0 && days_to_pay < 0 && years_to_pay <= 0;

                    if !is_late {
                        if days_to_pay == 0 {
                            defeated_now.push(count);
                        }
                        else if days_to_pay <= 10 {
                            defeated_10days.push(count);
                        }
                    }
                    else {
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
            }
            else if defeated_now.len() == 1 {
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
            }
            else if defeated_10days.len() == 1 {
                let count = defeated[0];
                let month_pay = (count.date_in.month() + count.paid_installments + 1) as i32 % 12;
                let months_to_pay = month_pay - now.month0() as i32;
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
            }
            else if defeated.len() == 1 {
                let count = defeated[0];
                let month_pay = (count.date_in.month() + count.paid_installments + 1) as i32 % 12;
                let months_to_pay = month_pay - now.month0() as i32;
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
        self.has_alerted = true;
    }
}
