use super::*;
use crate::chrono::NaiveDate;
use gtk::{Adjustment, Calendar, CheckButton, SpinButton};

pub fn get_add_box(stack: &Stack) -> Box {
    let box_add = Box::new(Orientation::Vertical, 10);

    let box_top = Box::new(Orientation::Horizontal, 10);
    box_top.add_css_class("title_i");

    let button_return = Button::new();
    button_return.add_css_class("link_return");

    button_return.set_label("Retornar");
    button_return.connect_clicked(clone!(@weak stack => move |_| {
        stack.set_visible_child_name("home");
    }));

    box_top.append(&Label::new(Some("Adicionando conta")));
    box_top.append(&button_return);

    let box_form = Box::new(Orientation::Vertical, 10);
    box_form.set_halign(gtk::Align::Center);

    let box_name = Box::new(Orientation::Vertical, 0);
    let box_title = Box::new(Orientation::Vertical, 0);
    let box_nature = Box::new(Orientation::Horizontal, 5);
    let box_date = Box::new(Orientation::Horizontal, 5);
    let box_description = Box::new(Orientation::Vertical, 0);
    let box_installments = Box::new(Orientation::Horizontal, 5);
    let box_status = Box::new(Orientation::Horizontal, 5);
    let box_value = Box::new(Orientation::Horizontal, 5);

    let name_label = Label::new(Some("*Name:"));
    let title_label = Label::new(Some("*Título:"));
    let nature_label = Label::new(Some("Natureza:"));
    let date_label = Label::new(Some("Data\ninicial:"));
    let description_label = Label::new(Some("Descrição:"));
    let installments_label = Label::new(Some("Nª de\nParcelas:"));
    let status_label = Label::new(Some("Já tá paga?"));
    let value_label = Label::new(Some("R$:\n(por parcela)"));

    let name_input = Entry::new();
    let title_input = Entry::new();
    let nature_input = DropDown::from_strings(&[
        "Casa",
        "Alimentação",
        "Transporte",
        "Saúde",
        "Lazer",
        "Receita",
        "Outros",
    ]);
    let date_input = Calendar::new();
    let date_button = Button::new();
    let description_input = Entry::new();
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

    name_input.set_css_classes(&["input"]);
    title_input.set_css_classes(&["input"]);
    date_input.set_css_classes(&["input", "date_input"]);
    date_button.set_css_classes(&["date_button", "input"]);
    date_input.set_visible(false);
    date_button.set_label(&date_string);

    date_input.set_size_request(10, 10);

    date_input.connect_day_selected(clone!(@weak date_button, @weak date_input => move |_| {
        let date_string = format!("{:02}/{:02}/{:04}", date_input.day(), date_input.month()+1, date_input.year());
        date_button.set_label(&date_string);
        date_input.set_visible(false);
        date_button.set_visible(true);
    }));

    date_button.connect_clicked(clone!(@weak date_button, @weak date_input => move |_| {
        date_button.set_visible(false);
        date_input.set_visible(true);
    }));

    box_name.set_halign(gtk::Align::Start);
    box_title.set_halign(gtk::Align::Start);
    box_description.set_halign(gtk::Align::Start);
    description_input.add_css_class("description_input");

    box_name.append(&name_label);
    box_name.append(&name_input);
    box_title.append(&title_label);
    box_title.append(&title_input);
    box_nature.append(&nature_label);
    box_nature.append(&nature_input);
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

    let button_append = Button::with_label("Adicionar");
    button_append.add_css_class("button");

    button_append.connect_clicked(clone!(
        @weak stack,
        @weak name_input,
        @weak title_input,
        @weak description_input,
        @weak date_input,
        @weak value_input,
        @weak status_input,
        @weak installment_input,
        @weak nature_input => move |_| {
            let mut count = Count::from(
                name_input.text().trim(),
                title_input.text().trim(),
                description_input.text().as_str(),
                value_input.value() as f32,
                NaiveDate::from_ymd_opt(date_input.year(), (date_input.month() + 1) as u32, date_input.day() as u32).unwrap(),
                installment_input.value() as u32,
                match nature_input.selected(){
                    0 => "Casa",
                    1 => "Alimentação",
                    2 => "Transporte",
                    3 => "Saúde",
                    4 => "Lazer",
                    5 => "Receita",
                    _ => "Outros"
                }
            );

            if status_input.is_active(){
                count.pay_all()
            }

            if !count.is_empty(){
                use tokio::runtime::Runtime;
                let rnt = Runtime::new().unwrap();
                get_counts_instance().put(count.clone());

                match rnt.block_on(control::save()){
                    Ok(_) => {
                        update_list(None, Some(&stack));
                        title_input.set_text("");
                        description_input.set_text("");
                        value_input.set_value(0.01);
                        date_input.clear_marks();
                        installment_input.set_value(1.0);
                        nature_input.set_selected(0);
                    },
                    Err(err) => println!("{err}")
                };
            }else{
                println!("Faltam dados!");
            }
    }));

    let grid = Grid::new();
    grid.set_column_homogeneous(true);
    grid.set_row_spacing(15);

    grid.attach(&box_name, 0, 0, 1, 1);
    grid.attach(&box_title, 0, 1, 1, 1);
    grid.attach(&box_description, 0, 2, 1, 1);
    grid.attach(&box_nature, 0, 3, 1, 1);
    grid.attach(&box_value, 0, 4, 1, 1);
    grid.attach(&box_installments, 0, 5, 1, 1);
    grid.attach(&box_date, 0, 6, 1, 1);
    grid.attach(&box_status, 0, 7, 1, 1);
    grid.attach(&button_append, 0, 8, 1, 1);

    box_form.append(&grid);

    box_add.append(&box_top);
    box_add.append(&box_form);

    box_add.set_overflow(gtk::Overflow::Hidden);
    box_add
}
