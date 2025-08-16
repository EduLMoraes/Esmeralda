use super::*;
use crate::{
    chrono::NaiveDate,
    prelude::{
        control::{add_people, get_peoples_instance},
        model::People::People,
    },
};
use gtk::{Adjustment, Calendar, CheckButton, SpinButton, TextView};

#[allow(deprecated)]
use gtk::ComboBoxText;

#[allow(deprecated)]
pub fn get_add_box(stack: &Stack) -> Box {
    let box_add = Box::new(Orientation::Vertical, 10);

    let box_top = Box::new(Orientation::Horizontal, 10);
    box_top.add_css_class("title_i");

    let button_return = Button::new();
    button_return.add_css_class("link_return");

    button_return.set_label("Retornar");
    button_return.connect_clicked(clone!(
        #[weak]
        stack,
        move |_| {
            stack.set_visible_child_name("home");
        }
    ));

    box_top.append(&Label::new(Some("Adicionando conta")));
    box_top.append(&button_return);

    let box_form = Box::new(Orientation::Vertical, 10);
    box_form.set_halign(gtk::Align::Center);

    let box_name = Box::new(Orientation::Vertical, 0);
    let box_title = Box::new(Orientation::Vertical, 0);
    let box_nature = Box::new(Orientation::Vertical, 5);
    let box_date = Box::new(Orientation::Vertical, 5);
    let box_description = Box::new(Orientation::Vertical, 0);
    let box_installments = Box::new(Orientation::Vertical, 5);
    let box_status = Box::new(Orientation::Horizontal, 5);
    let box_value = Box::new(Orientation::Vertical, 5);

    let name_label = Label::new(Some("*Name:"));
    let title_label = Label::new(Some("*Título:"));
    let nature_label = Label::new(Some("*Natureza:"));
    let date_label = Label::new(Some("Data inicial:"));
    let description_label = Label::new(Some("Descrição:"));
    let installments_label = Label::new(Some("Parcelas:"));
    let status_label = Label::new(Some("Já tá paga?"));
    let value_label = Label::new(Some("R$ p/ parcela"));

    name_label.set_halign(gtk::Align::Start);
    title_label.set_halign(gtk::Align::Start);
    nature_label.set_halign(gtk::Align::Start);
    date_label.set_halign(gtk::Align::Start);
    description_label.set_halign(gtk::Align::Start);
    status_label.set_halign(gtk::Align::Start);
    value_label.set_halign(gtk::Align::Start);

    let rnt = tokio::runtime::Runtime::new().unwrap();
    let natures_base = vec![
        String::from("Casa"),
        String::from("Transporte"),
        String::from("Alimentação"),
        String::from("Saúde"),
        String::from("Lazer"),
        String::from("Receita"),
        String::from("Outros"),
        String::from("+ Nova natureza"),
    ];
    let mut natures = match rnt.block_on(control::get_groups()) {
        Ok(groups) => groups,
        Err(err) => {
            tracing::error!("{:?}", err);
            natures_base.clone()
        }
    };

    for nature_base in natures_base {
        if !natures.contains(&nature_base) {
            natures.push(nature_base);
        }
    }

    natures.sort();

    let nature_input = ComboBoxText::new();
    for nature in natures {
        nature_input.append(None, &nature);
    }

    nature_input.set_active(Some(1));

    let name_input = ComboBoxText::new();
    for people in get_peoples_instance().iter() {
        name_input.append(None, &people.name);
    }

    name_input.set_active(Some(0));
    name_input.append(None, "+ Novo devedor");

    let new_nature_input = Entry::new();
    let new_name_input = Entry::new();
    let title_input = Entry::new();
    let date_input = Calendar::new();
    let date_button = Button::new();
    let description_input = TextView::new();
    let installment_input = SpinButton::new(
        Some(&Adjustment::new(0.0, 1.0, 999.0, 1.0, 0.1, 0.0)),
        1.0,
        0,
    );
    let status_input = CheckButton::new();
    let value_input = SpinButton::new(
        Some(&Adjustment::new(0.0, 0.01, 99999999.00, 0.01, 0.1, 0.0)),
        1.0,
        2,
    );

    let date_string = format!(
        "{:02}/{:02}/{:04}",
        date_input.day(),
        date_input.month() + 1,
        date_input.year()
    );

    name_input.set_css_classes(&["input", "name_input"]);
    title_input.set_css_classes(&["input", "title_input"]);
    date_input.set_css_classes(&["input", "date_input"]);
    date_button.set_css_classes(&["date_button", "input"]);
    date_input.set_visible(false);
    date_button.set_label(&date_string);

    let buffer = description_input.buffer();
    let start_iter = buffer.iter_at_offset(0);
    buffer.place_cursor(&start_iter);

    date_input.set_size_request(10, 10);

    nature_input.connect_changed(clone!(
        #[weak]
        nature_input,
        #[weak]
        new_nature_input,
        move |input| {
            if input.active_text().unwrap() == "+ Nova natureza" {
                new_nature_input.set_visible(true);
                nature_input.set_visible(false);
            }
        }
    ));

    name_input.connect_changed(clone!(
        #[weak]
        name_input,
        #[weak]
        new_name_input,
        move |input| {
            if input.active_text().unwrap() == "+ Novo devedor" {
                new_name_input.set_visible(true);
                name_input.set_visible(false);
            }
        }
    ));

    date_input.connect_day_selected(clone!(
        #[weak]
        date_button,
        #[weak]
        date_input,
        move |_| {
            let date_string = format!(
                "{:02}/{:02}/{:04}",
                date_input.day(),
                date_input.month() + 1,
                date_input.year()
            );
            date_button.set_label(&date_string);
            date_input.set_visible(false);
            date_button.set_visible(true);
        }
    ));

    date_button.connect_clicked(clone!(
        #[weak]
        date_button,
        #[weak]
        date_input,
        move |_| {
            date_button.set_visible(false);
            date_input.set_visible(true);
        }
    ));

    box_name.set_halign(gtk::Align::Start);
    box_title.set_halign(gtk::Align::Start);
    box_date.set_halign(gtk::Align::Start);
    box_description.set_halign(gtk::Align::Start);
    box_nature.set_halign(gtk::Align::Start);
    box_installments.set_halign(gtk::Align::Start);
    box_value.set_halign(gtk::Align::Start);
    description_input.add_css_class("description_input");
    new_nature_input.set_halign(gtk::Align::Start);

    box_name.append(&name_label);
    box_name.append(&name_input);
    box_name.append(&new_name_input);
    box_title.append(&title_label);
    box_title.append(&title_input);
    box_nature.append(&nature_label);
    box_nature.append(&nature_input);
    box_nature.append(&new_nature_input);
    box_date.append(&date_label);
    box_date.append(&date_button);
    box_date.append(&date_input);
    box_description.append(&description_label);
    box_description.append(&description_input);
    box_installments.append(&installments_label);
    box_installments.append(&installment_input);
    box_status.append(&status_label);
    box_status.append(&status_input);
    box_value.append(&value_label);
    box_value.append(&value_input);

    new_nature_input.set_visible(false);
    new_name_input.set_visible(false);

    let button_append = Button::with_label("Adicionar");
    button_append.add_css_class("button");

    button_append.connect_clicked(clone!(
        #[weak]
        stack,
        #[weak]
        name_input,
        #[weak]
        title_input,
        #[weak]
        description_input,
        #[weak]
        date_input,
        #[weak]
        value_input,
        #[weak]
        status_input,
        #[weak]
        installment_input,
        #[weak]
        nature_input,
        #[weak]
        new_nature_input,
        move |_| {
            let nature = if nature_input.active_text().unwrap() != "+ Nova natureza" {
                nature_input.active_text().unwrap().to_string()
            } else {
                new_nature_input.text().to_string()
            };

            let name = if name_input.active_text().unwrap() != "+ Novo devedor" {
                name_input.active_text().unwrap().to_string()
            } else {
                new_name_input.text().to_string()
            };

            tracing::info!("debtor {:?}", name);

            let natures_base = vec![
                String::from("Casa"),
                String::from("Transporte"),
                String::from("Alimentação"),
                String::from("Saúde"),
                String::from("Lazer"),
                String::from("Receita"),
                String::from("Outros"),
                String::from("+ Nova natureza"),
            ];

            let mut natures = match rnt.block_on(control::get_groups()) {
                Ok(groups) => groups,
                Err(err) => {
                    tracing::error!("{:?}", err);

                    natures_base.clone()
                }
            };

            for nature_base in natures_base {
                if !natures.contains(&nature_base) {
                    natures.push(nature_base);
                }
            }

            if !natures.contains(&nature) {
                nature_input.append(None, &nature);
            }

            let description = description_input.buffer();
            let title = title_input.text();

            // (value, input)
            let data: Vec<(&str, &str)> = vec![
                (title.trim(), "Título"),
                (nature.trim(), "Natureza"),
                (name.trim(), "Nome"),
            ];

            let mut count = Count::from(
                name.trim(),
                title.trim(),
                description
                    .text(&description.start_iter(), &description.end_iter(), true)
                    .as_str(),
                value_input.value() as f32,
                NaiveDate::from_ymd_opt(
                    date_input.year(),
                    (date_input.month() + 1) as u32,
                    date_input.day() as u32,
                )
                .unwrap(),
                installment_input.value() as u32,
                nature.trim(),
            );

            if status_input.is_active() {
                count.pay_all()
            }

            if !count.is_empty() {
                use tokio::runtime::Runtime;
                let rnt = Runtime::new().unwrap();

                get_counts_instance().put(count);

                match rnt.block_on(control::save()) {
                    Ok(_) => {
                        let new_people = People::new(&name);
                        let peoples = get_peoples_instance();

                        if !peoples.contains(&new_people) {
                            let response = rnt.block_on(add_people(&new_people));
                            name_input.append(None, &new_people.name);
                            if response.is_err() {
                                alert(
                                    "Erro ao tentar adicionar nova pessoa",
                                    "Falha ao adicionar pessoa",
                                );
                            }
                        }
                        reload_home(None, Some(&stack));
                        title_input.set_text("");
                        description_input.buffer().set_text("");
                        value_input.set_value(0.01);
                        date_input.clear_marks();
                        installment_input.set_value(1.0);
                        nature_input.set_active(Some(1));
                        nature_input.set_visible(true);
                        new_nature_input.set_visible(false);
                        new_nature_input.set_text("");
                        name_input.set_active(Some(0));
                        name_input.set_visible(true);
                        new_name_input.set_visible(false);
                        new_name_input.set_text("");
                    }
                    Err(err) => tracing::error!("{err}"),
                };
            } else {
                for (value, input) in data {
                    if value.is_empty() {
                        use alerts::alert;
                        alert(
                            &format!("Campo obrigatório {input} está vazio."),
                            "Faltam dados!",
                        );
                    }
                }
            }
        }
    ));

    let grid = Grid::new();
    grid.set_column_homogeneous(true);
    grid.set_row_spacing(15);
    grid.add_css_class("grid_add");

    grid.attach(&box_name, 0, 0, 1, 1);
    grid.attach(&box_nature, 0, 1, 1, 1);
    grid.attach(&box_date, 0, 3, 1, 1);
    grid.attach(&box_installments, 1, 3, 1, 1);
    grid.attach(&box_title, 0, 4, 1, 1);
    grid.attach(&box_value, 1, 4, 1, 1);
    grid.attach(&box_description, 0, 5, 2, 1);
    grid.attach(&box_status, 0, 6, 1, 1);
    grid.attach(&button_append, 1, 7, 1, 1);

    box_form.append(&grid);

    box_add.append(&box_top);
    box_add.append(&box_form);

    box_add.set_overflow(gtk::Overflow::Hidden);
    box_add
}
