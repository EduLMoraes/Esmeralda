use std::str::FromStr;

use crate::utils::validate::date_valid;

use super::*;
use chrono::NaiveDate;
use control::{edit_people, edit_user, get_peoples_instance};
use gtk::{Align, Entry};

pub fn get_box() -> Box {
    let box_index = Box::new(Orientation::Horizontal, 2);
    box_index.add_css_class("box_config");

    let user_instance = get_user_instance().clone().unwrap();
    let peoples_instance = get_peoples_instance();

    let grid = Grid::new();
    grid.set_column_homogeneous(true);
    grid.set_row_homogeneous(false);

    let title = Label::new(Some("Configurações de usuário"));
    title.set_halign(Align::Center);
    title.set_css_classes(&["title_configs"]);

    grid.attach(&title, 0, 0, 2, 1);

    let box_username = BoxConfigUser::new("Username: ", Some(&user_instance.username));
    let box_email = BoxConfigUser::new("Email: ", Some(&user_instance.email));

    grid.attach(&box_username.box_config, 0, 1, 1, 1);
    grid.attach(&box_email.box_config, 1, 1, 1, 1);

    for n in 0..peoples_instance.len() {
        let box_people_n = Box::new(Orientation::Vertical, 1);
        let subtitle = Label::new(Some(&format!("Pessoa nª: {}", n + 1)));
        subtitle.add_css_class("subtitle_people_config");
        let box_name = BoxConfigUser::new_with_index(n, "Nome: ", Some(&peoples_instance[n].name));
        let box_surname =
            BoxConfigUser::new_with_index(n, "Sobrenome: ", Some(&peoples_instance[n].surname));
        let box_cell_phone =
            BoxConfigUser::new_with_index(n, "Celular: ", Some(&peoples_instance[n].cell_phone));
        let box_birthday = BoxConfigUser::new_with_index(
            n,
            "Nascimento: ",
            Some(&format!(
                "{:02}/{:02}/{:04}",
                &peoples_instance[n].birthday.day0() + 1,
                &peoples_instance[n].birthday.month0() + 1,
                &peoples_instance[n].birthday.year()
            )),
        );

        box_name.input.connect_activate(move |input| {
            let mut peoples = get_peoples_instance();

            if input.text().is_empty() {
                alert("Esse campo não pode estar vazio.", "Entrada inválida");
                input.set_text(&peoples[box_name.index].name);
                return;
            } else {
                peoples[box_name.index].name = input.text().to_string();
                let rnt = tokio::runtime::Runtime::new().unwrap();
                let _ = rnt
                    .block_on(edit_people(peoples.to_vec()))
                    .map_err(|err| dbg!(err));

                alert("Nome alterado com sucesso!", "Sucesso");
            }
        });

        box_cell_phone.input.connect_activate(move |input| {
            let mut peoples = get_peoples_instance();

            if input.text().is_empty() {
                alert("Esse campo não pode estar vazio.", "Entrada inválida");
                input.set_text(&peoples[box_cell_phone.index].cell_phone);
                return;
            } else {
                peoples[box_cell_phone.index].cell_phone = input.text().to_string();
                let rnt = tokio::runtime::Runtime::new().unwrap();
                let _ = rnt
                    .block_on(edit_people(peoples.to_vec()))
                    .map_err(|err| dbg!(err));

                alert("Celular alterado com sucesso!", "Sucesso");
            }
        });

        box_birthday.input.connect_activate(move |input| {
            let mut peoples = get_peoples_instance();

            if input.text().is_empty() {
                alert("Esse campo não pode estar vazio.", "Entrada inválida");
                input.set_text(&format!(
                    "{:02}/{:02}/{:04}",
                    &peoples[box_birthday.index].birthday.day0() + 1,
                    &peoples[box_birthday.index].birthday.month0() + 1,
                    &peoples[box_birthday.index].birthday.year()
                ));
                return;
            } else {
                let mut text = input.text().to_string();
                if (text.contains("/") && !date_valid::validate(&text))
                    || (!text.contains("/") && text.len() != 8)
                {
                    alert(
                        "Revise o formato da data e se ela está correta.",
                        "Data inválida",
                    );
                    return;
                }

                while let Some(idx) = text.find("/") {
                    text.remove(idx);
                }

                let day = &text[0..2];
                let month = &text[2..4];
                let year = &text[4..8];

                let day = day.parse::<u16>();
                let month = month.parse::<u16>();
                let year = year.parse::<u16>();

                if day.is_err() || month.is_err() || year.is_err() {
                    alert(
                        "Revise o formato da data e se ela está correta.",
                        "Data inválida",
                    );
                    return;
                }

                let new_date = NaiveDate::from_str(&format!(
                    "{}-{}-{}",
                    year.unwrap(),
                    month.unwrap(),
                    day.unwrap()
                ))
                .map_err(|e| dbg!(e));

                if new_date.is_err() {
                    alert(
                        "Revise o formato da data e se ela está correta.",
                        "Data inválida",
                    );
                    return;
                }

                peoples[box_birthday.index].birthday = new_date.unwrap();
                let rnt = tokio::runtime::Runtime::new().unwrap();
                let _ = rnt
                    .block_on(edit_people(peoples.to_vec()))
                    .map_err(|err| dbg!(err));

                input.set_text(&format!(
                    "{:02}/{:02}/{:04}",
                    &peoples[box_birthday.index].birthday.day0() + 1,
                    &peoples[box_birthday.index].birthday.month0() + 1,
                    &peoples[box_birthday.index].birthday.year()
                ));

                alert("Nascimento alterado com sucesso!", "Sucesso");
            }
        });

        box_surname.input.connect_activate(move |input| {
            let mut peoples = get_peoples_instance();

            if input.text().is_empty() {
                alert("Esse campo não pode estar vazio.", "Entrada inválida");
                input.set_text(&peoples[box_surname.index].surname);
                return;
            } else {
                peoples[box_surname.index].surname = input.text().to_string();
                let rnt = tokio::runtime::Runtime::new().unwrap();
                let _ = rnt
                    .block_on(edit_people(peoples.to_vec()))
                    .map_err(|err| dbg!(err));

                alert("Sobrenome alterado com sucesso!", "Sucesso");
            }
        });

        box_people_n.append(&subtitle);
        box_people_n.append(&box_name.box_config);
        box_people_n.append(&box_surname.box_config);
        box_people_n.append(&box_cell_phone.box_config);
        box_people_n.append(&box_birthday.box_config);

        grid.attach(&box_people_n, (n as i32) % 2, ((n as i32) % 2) + 2, 1, 1);
    }

    box_username.input.connect_activate(move |input| {
        let mut user = get_user_instance().clone().unwrap();

        if input.text().is_empty() {
            alert("Esse campo não pode estar vazio.", "Entrada inválida");
            input.set_text(user.get_username());
            return;
        } else {
            user.username = input.text().to_string();
            let rnt = tokio::runtime::Runtime::new().unwrap();
            let _ = rnt.block_on(edit_user(user)).map_err(|err| dbg!(err));

            alert("Username alterado com sucesso!", "Sucesso");
        }
    });

    box_email.input.connect_activate(move |input| {
        let mut user = get_user_instance().clone().unwrap();

        if input.text().is_empty() {
            alert("Esse campo não pode estar vazio.", "Entrada inválida");
            input.set_text(user.get_email());
            return;
        } else {
            user.email = input.text().to_string();
            let rnt = tokio::runtime::Runtime::new().unwrap();
            let _ = rnt.block_on(edit_user(user)).map_err(|err| dbg!(err));

            alert("email alterado com sucesso!", "Sucesso");
        }
    });

    box_index.append(&grid);
    box_index
}

struct BoxConfigUser {
    index: usize,
    box_config: Box,
    input: Entry,
}

impl BoxConfigUser {
    pub fn new(label: &str, value: Option<&str>) -> Self {
        let label = Label::new(Some(label));
        label.set_css_classes(&["label_configs"]);
        label.set_halign(Align::Start);

        let box_default = Box::builder()
            .orientation(Orientation::Horizontal)
            .spacing(0)
            .css_classes(["box_input"])
            .halign(Align::Start)
            .valign(Align::Fill)
            .hexpand(false)
            .vexpand(false)
            .build();

        let input_default = Entry::builder()
            .hexpand(false)
            .vexpand(false)
            .halign(Align::End)
            .valign(Align::Fill)
            .css_classes(["input_configs"])
            .text(value.unwrap_or(""))
            .build();

        box_default.append(&label);
        box_default.append(&input_default);

        Self {
            index: 0,
            box_config: box_default,
            input: input_default,
        }
    }

    pub fn new_with_index(idx: usize, label: &str, value: Option<&str>) -> Self {
        let mut bx = Self::new(label, value);
        bx.set_index(idx);
        bx
    }

    pub fn set_index(&mut self, idx: usize) {
        self.index = idx;
    }
}
