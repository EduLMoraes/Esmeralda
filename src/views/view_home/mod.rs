use crate::{
    controller::{
        data_controller::{recover, recover_years},
        people_controller::get_peoples_instance,
        user_controller::{exit_user, get_user_instance},
    },
    model::{count::Count, list::get_counts_instance},
    views::{
        alerts::alert,
        login::ViewLogin,
        register::ViewRegister,
        view_home::{
            box_count::BoxCount, box_investment::Investments, box_left_menu::LeftMenu,
            box_plot::Plot,
        },
    },
};
use chrono::{Datelike, Utc};
use glib::clone;
use gtk::{
    prelude::*, Align, Box, Button, DropDown, Image, Label, Orientation, SearchEntry, Stack,
};
use std::{borrow::BorrowMut, cell::RefCell, env};
use tokio::runtime::Runtime;

mod box_count;
mod box_investment;
mod box_left_menu;
mod box_plot;
mod box_user;

#[derive(Clone, Copy, Debug)]
pub enum PageHome {
    Count,
    Investments,
    Plot,
    User,
}

#[derive(Clone)]
pub struct HomeView {
    pub global_stack: RefCell<Stack>,
    pub home_stack: Stack,
    pub current_page: Option<PageHome>,
    pub has_alerted: bool,
}

impl HomeView {
    pub fn new(global_stack: RefCell<Stack>) -> HomeView {
        HomeView {
            global_stack,
            home_stack: Stack::new(),
            current_page: None,
            has_alerted: false,
        }
    }

    pub fn load_page(&mut self, page: PageHome) {
        self.current_page = Some(page);
        let mut page: std::boxed::Box<dyn IsBoxView> = match page {
            PageHome::Plot => std::boxed::Box::new(Plot::new()),
            PageHome::Investments => std::boxed::Box::new(Investments::new()),
            _ => std::boxed::Box::new(BoxCount::new(RefCell::new(self.clone()))),
        };

        let current_child = self.home_stack.first_child();
        if let Some(current_child) = current_child {
            self.home_stack.remove(&current_child);
        }

        self.home_stack
            .add_titled(&page.get_box(), Some(page.get_title()), page.get_title());
        self.home_stack.set_visible_child_name(page.get_title());
    }

    pub fn home_screen(&self) -> Box {
        self.global_stack
            .borrow()
            .set_css_classes(&["home_window", "window"]);

        let screen = Box::new(Orientation::Horizontal, 0);

        let run = tokio::runtime::Runtime::new().unwrap();

        if let Ok(_years) = run.block_on(recover_years()) {
            let _ = run
                .block_on(recover(Utc::now().year() as i16))
                .map_err(|err| tracing::error!("{err:?}"));
        }

        self.perfomance_resume();
        self.alert_dead_line();

        screen.set_valign(Align::Center);
        screen.set_halign(Align::Center);
        let box_body = self.clone().get_box_body();
        box_body.append(&self.home_stack);

        screen.append(&LeftMenu::new(RefCell::new(self.clone())).get_box());
        screen.append(&box_body);

        screen
    }

    pub fn reload_home(&mut self) {
        let curr_page = *self.current_page.as_ref().unwrap();
        tracing::info!("curr_page {curr_page:?}");
        self.load_page(curr_page);
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
        select_year.connect_selected_item_notify(clone!(
            #[strong(rename_to = slf)]
            self,
            move |select_year| {
                let mut home = slf.clone();
                let counts = get_counts_instance().years.clone();

                let rnt = Runtime::new().unwrap();
                rnt.block_on(recover(counts[select_year.selected() as usize]))
                    .unwrap();

                home.reload_home();
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
            #[strong(rename_to=slf)]
            self,
            move |_| {
                let mut home = slf.clone();
                home.borrow_mut().reload_home();
            }
        ));

        let button_ext = Button::with_label("Sair");
        button_ext.set_halign(gtk::Align::Center);
        button_ext.set_valign(gtk::Align::Center);
        button_ext.add_css_class("link_button");

        button_ext.connect_clicked(clone!(
            #[strong(rename_to = slf)]
            self,
            move |_| {
                let login_screen = ViewLogin::new(slf.global_stack.clone());
                let register_screen = ViewRegister::new(slf.global_stack.clone());

                slf.global_stack.borrow().add_titled(
                    &login_screen.login_screen(),
                    Some("login"),
                    "Login",
                );
                slf.global_stack.borrow().add_titled(
                    &register_screen.rgter_screen(),
                    Some("register"),
                    "Register",
                );

                slf.global_stack.borrow().set_visible_child_name("login");

                let tmp = slf.global_stack.borrow().child_by_name("home").unwrap();
                slf.global_stack.borrow().remove(&tmp);

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
            #[strong(rename_to = slf)]
            self,
            move |_| {
                let mut home = slf.clone();
                home.load_page(PageHome::User);
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

    pub fn get_box_body(&mut self) -> Box {
        let box_body = Box::new(Orientation::Vertical, 0);

        self.load_page(PageHome::Count); // 2. Empréstimo mutável (&mut self) acontece e termina aqui.

        // 3. Agora podemos usar tudo, pois os empréstimos anteriores já acabaram.
        box_body.append(&self.box_top());
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

    pub fn alert_dead_line(&self) {
        if !self.has_alerted {
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
                    let days_to_pay =
                        (count.date_out.day() as i32 + (months_to_pay * 30)) - now.day() as i32;
                    let years_to_pay = count.date_out.year() - now.year();
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
                let months_to_pay = month_pay - now.month() as i32;
                let days_to_pay =
                    (count.date_out.day0() as i32 + (months_to_pay * 30)) - now.day() as i32;

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
                let months_to_pay = month_pay - now.month() as i32;
                let days_to_pay =
                    (count.date_out.day0() as i32 + (months_to_pay * 30)) - now.day() as i32;

                alert(
                    &format!(
                        "A conta {} venceu há {}. Atente-se ao prazo!",
                        count.title, days_to_pay
                    ),
                    "Vencimento de contas",
                );
            }
        }
        // self.has_alerted = true;
    }
}

pub trait IsBoxView {
    fn get_box(&mut self) -> Box;
    fn get_title(&self) -> &'static str;
}
