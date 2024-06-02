use super::*;
use crate::control::GLOBAL_COUNTS;

#[allow(deprecated)]
pub fn edit_count(title: &str, count: &Count) -> MessageDialog {
    let box_form = Box::new(Orientation::Vertical, 10);
    box_form.set_halign(gtk::Align::Center);

    // The boxes of form
    let box_name = Box::new(Orientation::Vertical, 0);
    let box_title = Box::new(Orientation::Vertical, 0);
    let box_nature = Box::new(Orientation::Horizontal, 5);
    let box_date = Box::new(Orientation::Horizontal, 5);
    let box_description = Box::new(Orientation::Vertical, 0);
    let box_installments = Box::new(Orientation::Horizontal, 5);
    let box_status = Box::new(Orientation::Horizontal, 5);
    let box_value = Box::new(Orientation::Horizontal, 5);
    let box_buttons = Box::new(Orientation::Horizontal, 5);

    // The labels of inputs.
    let name_label = Label::new(Some("*Name:"));
    let title_label = Label::new(Some("*Título:"));
    let nature_label = Label::new(Some("Natureza:"));
    let date_label = Label::new(Some("Data\ninicial:"));
    let description_label = Label::new(Some("Descrição:"));
    let installments_label = Label::new(Some("Nª de\nParcelas:"));
    let status_label = Label::new(Some("Já tá paga?"));
    let value_label = Label::new(Some("R$:\n(por parcela)"));

    // Natures existents
    let natures = vec![
        "Casa",
        "Alimentação",
        "Transprte",
        "Saúde",
        "Lazer",
        "Receita",
        "Outros",
    ];

    // Inputs of form.
    let name_in = Entry::new();
    let title_in = Entry::new();
    let nature_in = DropDown::from_strings(&natures);
    let date_in = Calendar::new();
    let date_button = Button::new();
    let description_in = Entry::new();
    let installment_in = SpinButton::new(
        Some(&Adjustment::new(0.0, 1.0, 999.0, 1.0, 0.1, 0.0)),
        1.0,
        0,
    );
    let status_in = CheckButton::new();
    let value_in = SpinButton::new(
        Some(&Adjustment::new(0.0, 0.01, 99999999.00, 0.01, 0.1, 0.0)),
        1.0,
        2,
    );

    // Format of date and index to nature.
    let date_string = format!(
        "{:02}/{:02}/{:04}",
        &count.date_in.day(),
        &count.date_in.month(),
        &count.date_in.year()
    );

    let mut i = 0;
    for nat in &natures {
        if nat == &count.nature {
            break;
        }
        i += 1;
    }

    // Set values of inputs with the data of count.
    name_in.set_text(&count.debtor);
    title_in.set_text(&count.title);
    description_in.set_text(&count.description);
    nature_in.set_selected(i);
    installment_in.set_value(count.installments as f64);
    value_in.set_value(count.value as f64);
    status_in.set_active(count.status);

    // Styles.
    description_in.add_css_class("description_input");
    name_in.set_css_classes(&["input"]);
    title_in.set_css_classes(&["input"]);
    date_in.set_css_classes(&["input", "date_input"]);
    date_button.set_css_classes(&["date_button", "input"]);
    date_in.set_visible(false);
    date_button.set_label(&date_string);

    date_in.set_size_request(10, 10);

    box_name.set_halign(gtk::Align::Start);
    box_title.set_halign(gtk::Align::Start);
    box_description.set_halign(gtk::Align::Start);

    // Functions of inputs.
    date_in.connect_day_selected(clone!(@weak date_button, @weak date_in => move |_| {
        let date_string = format!("{:02}/{:02}/{:04}", date_in.day(), date_in.month()+1, date_in.year());
        date_button.set_label(&date_string);
        date_in.set_visible(false);
        date_button.set_visible(true);
    }));

    date_button.connect_clicked(clone!(@weak date_button, @weak date_in => move |_| {
        date_button.set_visible(false);
        date_in.set_visible(true);
    }));

    // Buttons of confirm and cancel.
    let cancel = Button::with_label("Cancelar");
    let confirm = Button::with_label("Confirmar");
    confirm.add_css_class("button");

    // Using all.
    box_name.append(&name_label);
    box_name.append(&name_in);
    box_title.append(&title_label);
    box_title.append(&title_in);
    box_nature.append(&nature_label);
    box_nature.append(&nature_in);
    box_date.append(&date_label);
    box_date.append(&date_button);
    box_date.append(&date_in);
    box_description.append(&description_label);
    box_description.append(&description_in);
    box_installments.append(&installments_label);
    box_installments.append(&installment_in);
    box_status.append(&status_label);
    box_status.append(&status_in);
    box_value.append(&value_label);
    box_value.append(&value_in);
    box_buttons.append(&cancel);
    box_buttons.append(&confirm);

    box_form.append(&box_name);
    box_form.append(&box_title);
    box_form.append(&box_nature);
    box_form.append(&box_date);
    box_form.append(&box_description);
    box_form.append(&box_installments);
    box_form.append(&box_status);
    box_form.append(&box_value);
    box_form.append(&box_buttons);

    let edit = MessageDialog::builder()
        .buttons(gtk::ButtonsType::OkCancel)
        .message_type(gtk::MessageType::Question)
        .child(&box_form)
        .title(title)
        .overflow(gtk::Overflow::Visible)
        .build();

    cancel.connect_clicked(clone!( @weak edit => move |_| edit.close()));
    confirm.connect_clicked(clone!( @weak edit, @strong count => move |_| {
        use crate::chrono::NaiveDate;

        let mut test = Count::from(
            &name_in.text(),
            &title_in.text(),
            &description_in.text(),
            value_in.value() as f32,
            NaiveDate::from_ymd_opt(date_in.year(), (date_in.month() + 1) as u32, date_in.day() as u32).unwrap(),
            installment_in.value() as u32,
            natures[nature_in.selected() as usize]
        );

        if status_in.is_active(){
            test.pay_all();
        }

        unsafe{
            let tmp = GLOBAL_COUNTS.borrow_mut();
            for i in 0..tmp.list.len(){
                if tmp.list[i].id == count.id{
                    tmp.list[i] = test.clone();
                    tmp.list[i].id = count.id;
                    break;
                }
            }
            use tokio::runtime::Runtime;
            let rnt = Runtime::new().unwrap();

            match rnt.block_on(control::edit(tmp)){
                Ok(_) => {},
                Err(err) => println!("{err}")
            };
        }

        edit.close()
    }));

    edit
}
