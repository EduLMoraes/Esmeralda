use super::*;

mod bottom_left;
mod bottom_right;
mod box_add;
mod box_debtor;
mod box_group;
mod box_home;
mod box_info;
mod box_month;
mod box_pay;

mod grids;
use grids::get_grid_infos;

pub use bottom_left::*;
pub use bottom_right::*;
pub use box_add::*;
pub use box_debtor::*;
pub use box_group::*;
pub use box_home::*;
pub use box_info::*;
pub use box_month::*;
pub use box_pay::*;

#[allow(static_mut_refs)]


pub struct BoxCount {
    pub stack: &Stack,
}

impl BoxCount {
    pub fn new() -> Box {
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
    pub fn new_box_info(info: &Count) -> Box {
        let box_info = Box::new(Orientation::Vertical, 0);
        box_info.add_css_class("box_new_info");

        let box_top = Box::new(Orientation::Horizontal, 0);
        let box_body = Box::new(Orientation::Horizontal, 0);
        let box_bottom = Box::new(Orientation::Horizontal, 0);
        box_bottom.set_halign(gtk::Align::Center);

        let box_left_i = Box::new(Orientation::Vertical, 2);
        box_left_i.add_css_class("box_left_i");

        let name = Label::new(Some(&info.debtor));
        name.add_css_class("name_i");
        let mut icon_path = PathBuf::from(format!("{}info_icon", var("ICON_PATH").unwrap()));
        icon_path.push(format!("{}.png", info.nature.to_lowercase()));

        if !icon_path.exists() {
            icon_path = PathBuf::from(format!(
                "{}info_icon/not_found.png",
                var("ICON_PATH").unwrap()
            ));
        }

        let icon = Image::from_file(icon_path);
        icon.add_css_class("icon_info");

        if info.status {
            icon.add_css_class("positive");
        } else {
            icon.add_css_class("negative");
        }

        box_top.append(&name);
        box_left_i.append(&icon);
        box_left_i.set_halign(gtk::Align::Start);

        let box_center_i = Box::new(Orientation::Vertical, 2);
        box_center_i.add_css_class("box_center_i");
        box_center_i.set_valign(gtk::Align::Center);
        box_center_i.set_hexpand(true);

        let value = if info.nature == *"Receita" {
            format!("R$ +{:.2}", info.value)
        } else {
            format!("R$ -{:.2}", info.value)
        };
        let label_value = Label::new(Some(&value));
        label_value.add_css_class("label_value_i");

        let title = Label::new(Some(&format!("{:.40}", info.title)));
        title.add_css_class("title_i");

        box_center_i.append(&label_value);
        box_bottom.append(&title);

        let box_right_i = Box::new(Orientation::Vertical, 2);
        box_right_i.add_css_class("box_right_i");

        let label_status = Label::new(Some(""));
        label_status.add_css_class("label_status_i");

        if info.status {
            label_status.set_label("Paga");
            label_status.add_css_class("status_positive");
        } else {
            label_status.set_label("Devendo");
            label_status.add_css_class("status_negative");
        }

        let date = Label::new(Some(&format!(
            "{:02}/{:02}/{:02} - {:02}/{:02}/{:02}",
            &info.date_in.day(),
            &info.date_in.month(),
            &info.date_in.year().to_string().get(2..=3).unwrap(),
            &info.date_out.day(),
            &info.date_out.month(),
            &info.date_out.year().to_string().get(2..=3).unwrap()
        )));

        date.add_css_class("date_i");

        let gesture = GestureClick::new();
        gesture.set_button(BUTTON_SECONDARY);

        gesture.connect_pressed(clone!(
            #[strong]
            info,
            #[weak]
            box_info,
            move |_, _, _, _| {
                let button_del = Button::with_label("Deletar");
                let button_edt = Button::with_label("Editar");

                let box_options = Box::new(Orientation::Vertical, 1);
                box_options.append(&button_edt);
                box_options.append(&button_del);

                let options = PopoverMenu::builder().child(&box_options).build();

                button_del.connect_clicked(clone!(
                    #[strong]
                    info,
                    #[weak]
                    options,
                    move |_| {
                        options.popdown();

                        let alert_confirm =
                            confirm("Tem certeza que deseja deletar a conta?", "Atenção");
                        if alert_confirm.is_some() {
                            let alert_confirm = alert_confirm.unwrap();
                            alert_confirm.present();

                            #[allow(deprecated)]
                            alert_confirm.connect_response(clone!(move |_, res| {
                                match res {
                                    ResponseType::Yes => {
                                        if !get_counts_instance().remove(&info.id) {
                                            alert("Ocorreu um erro ao tentar", "Erro!");
                                        } else {
                                            reload_home(None, None)
                                        }
                                    }
                                    ResponseType::No => {
                                        println!("Ação cancelada!");
                                    }
                                    _ => {}
                                }
                            }));
                        }
                    }
                ));

                button_edt.connect_clicked(clone!(
                    #[strong]
                    info,
                    #[weak]
                    options,
                    move |_| {
                        options.popdown();

                        let form = edit_count("Editar conta", &info);
                        if form.is_some() {
                            let form = form.unwrap();

                            form.connect_destroy(|_| {
                                reload_home(None, None);
                            });

                            form.present();
                        }
                    }
                ));

                box_info.append(&options);
                options.popup();
            }
        ));

        box_right_i.append(&label_status);
        box_right_i.append(&date);
        box_right_i.set_valign(gtk::Align::End);

        box_body.append(&box_left_i);
        box_body.append(&box_center_i);
        box_body.append(&box_right_i);

        box_info.add_controller(gesture);
        box_info.append(&box_top);
        box_info.append(&box_body);
        box_info.append(&box_bottom);

        box_info
    }

    pub fn box_info(info: &Count, stack: &Stack) -> Box {
        let box_info = Box::new(Orientation::Vertical, 0);
        box_info.add_css_class("box_info");

        let box_top = Box::new(Orientation::Horizontal, 0);
        box_top.set_halign(gtk::Align::Fill);
        box_top.set_hexpand(true);
        let box_body = Box::new(Orientation::Horizontal, 0);
        box_body.set_halign(gtk::Align::Fill);
        box_body.set_hexpand(true);

        let box_left_i = Box::new(Orientation::Vertical, 2);
        box_left_i.add_css_class("box_left_info");
        box_left_i.set_valign(gtk::Align::Center);

        let name = Label::new(Some(&format!("{:.10}", &info.debtor)));
        name.add_css_class("name_i");
        let mut icon_path = PathBuf::from(format!("{}info_icon", var("ICON_PATH").unwrap()));
        icon_path.push(format!("{}.png", info.nature.to_lowercase()));

        if !icon_path.exists() {
            icon_path = PathBuf::from(format!(
                "{}info_icon/not_found.png",
                var("ICON_PATH").unwrap()
            ));
        }

        let icon = Image::from_file(icon_path);
        icon.add_css_class("icon_info_box");

        if info.status {
            icon.add_css_class("positive");
        } else {
            icon.add_css_class("negative");
        }

        box_top.append(&name);
        box_left_i.append(&icon);
        box_left_i.set_halign(gtk::Align::Start);

        let box_center_i = Box::new(Orientation::Vertical, 2);
        box_center_i.add_css_class("box_center_info");
        box_center_i.set_valign(gtk::Align::Center);
        box_center_i.set_hexpand(true);

        let value = format!("R$ {:.2}", info.value);
        let label_value = Label::new(Some(&value));
        label_value.add_css_class("label_value_i");

        let title = Label::new(Some(&format!(
            "{:.10}\t{}/{}",
            info.title, info.paid_installments, info.installments
        )));
        title.add_css_class("title_i");

        box_center_i.append(&label_value);
        box_center_i.append(&title);

        let box_right_i = Box::new(Orientation::Vertical, 2);
        box_right_i.add_css_class("box_right_info");

        let label_status = Label::new(Some(""));
        let button_status = Button::new();
        label_status.add_css_class("label_status_i");
        button_status.add_css_class("button_status_negative");
        button_status.add_css_class("button");

        button_status.connect_clicked(clone!(
            #[strong]
            info,
            #[weak]
            stack,
            move |_| {
                use crate::tokio::runtime::Runtime;

                get_counts_instance().pay(info.id);
                let ref_counts = get_counts_instance().clone();

                let rn = Runtime::new().unwrap();

                rn.block_on(edit(&ref_counts)).unwrap();

                rn.block_on(update_counts_with_db()).ok();
                reload_home(None, Some(&stack));
            }
        ));

        let date = Label::new(Some(&format!(
            "{:02}/{:02}/{} - {:02}/{:02}/{:02}",
            &info.date_in.day(),
            &info.date_in.month(),
            &info.date_in.year().to_string().get(2..=3).unwrap(),
            &info.date_out.day(),
            &info.date_out.month(),
            &info.date_out.year().to_string().get(2..=3).unwrap()
        )));

        date.add_css_class("date_i");

        box_right_i.append(&label_status);
        box_right_i.append(&date);
        box_right_i.set_valign(gtk::Align::End);

        if info.status {
            label_status.set_label("Paga");
            label_status.add_css_class("status_positive");
        } else {
            label_status.set_label("Devendo");
            label_status.add_css_class("status_negative");
            box_right_i.append(&button_status);
            button_status.set_label("pagar");
        }

        let gesture = GestureClick::new();
        gesture.set_button(BUTTON_SECONDARY);

        gesture.connect_pressed(clone!(
            #[strong]
            info,
            #[weak]
            box_info,
            #[weak]
            stack,
            move |_, _, _, _| {
                let button_del = Button::with_label("Deletar");
                let button_edt = Button::with_label("Editar");

                let box_options = Box::new(Orientation::Vertical, 1);
                box_options.append(&button_edt);
                box_options.append(&button_del);

                let options = PopoverMenu::builder().child(&box_options).build();

                button_del.connect_clicked(clone!(
                    #[strong]
                    info,
                    #[weak]
                    options,
                    #[weak]
                    stack,
                    move |_| {
                        options.popdown();
                        let alert_confirm =
                            confirm("Tem certeza que deseja deletar a conta?", "Atenção");

                        if alert_confirm.is_some() {
                            let alert_confirm = alert_confirm.unwrap();
                            alert_confirm.present();

                            #[allow(deprecated)]
                            alert_confirm.connect_response(clone!(
                                #[weak]
                                stack,
                                move |_, res| {
                                    match res {
                                        ResponseType::Yes => {
                                            if !get_counts_instance().remove(&info.id) {
                                                alert("Ocorreu um erro ao tentar", "Erro!");
                                            } else {
                                                reload_home(None, Some(&stack))
                                            }
                                        }
                                        ResponseType::No => {
                                            println!("Ação cancelada!");
                                        }
                                        _ => {}
                                    }
                                }
                            ));
                        }
                    }
                ));

                button_edt.connect_clicked(clone!(
                    #[strong]
                    info,
                    #[weak]
                    options,
                    #[weak]
                    stack,
                    move |_| {
                        options.popdown();

                        let form = edit_count("Editar conta", &info);
                        if form.is_some() {
                            let form = form.unwrap();

                            form.connect_destroy(move |_| {
                                reload_home(None, Some(&stack));
                            });

                            form.present();
                        }
                    }
                ));

                box_info.append(&options);
                options.popup();
            }
        ));

        box_body.add_controller(gesture);
        box_body.append(&box_left_i);
        box_body.append(&box_center_i);
        box_body.append(&box_right_i);

        box_info.append(&box_top);
        box_info.append(&box_body);
        box_info
    }
    pub fn get_home_box(stack: &Stack) -> Box {
        let box_home = Box::new(Orientation::Vertical, 10);
        box_home.add_css_class("box_left_bb");
        box_home.set_hexpand(true);
        box_home.set_vexpand(true);
        box_home.set_valign(gtk::Align::Fill);
        box_home.set_halign(gtk::Align::Fill);

        let box_button_lb = Box::new(Orientation::Horizontal, 10);
        box_button_lb.set_halign(gtk::Align::Center);

        let button_add = Button::with_label("Adicionar");
        let button_payment = Button::with_label("Pagar");

        button_add.add_css_class("button_add");
        button_payment.add_css_class("button_payment");

        button_add.connect_clicked(clone!(
            #[weak]
            stack,
            move |_| {
                stack.set_visible_child_name("addition");
            }
        ));

        button_payment.connect_clicked(clone!(
            #[weak]
            stack,
            move |_| {
                stack.set_visible_child_name("payment");
            }
        ));

        box_button_lb.append(&button_add);
        box_button_lb.append(&button_payment);

        let stack_infos = Stack::new();
        let stack_switcher = StackSwitcher::new();
        stack_switcher.add_css_class("stack_switcher");

        stack_infos.add_titled(
            &get_grid_groups(&stack_infos, stack),
            Some("groups"),
            "Contas",
        );
        stack_infos.add_titled(&get_grid_debtors(), Some("debtors"), "Devedores");
        stack_infos.add_titled(
            &get_grid_months(&stack_infos, stack),
            Some("months"),
            "Meses",
        );
        stack_infos.set_visible_child_name("groups");

        stack_switcher.set_stack(Some(&stack_infos));

        let box_head_stack = Box::new(Orientation::Horizontal, 5);
        box_head_stack.append(&stack_switcher);

        let box_stack = Box::new(Orientation::Vertical, 5);
        box_stack.append(&box_head_stack);
        box_stack.append(&stack_infos);

        let scrolled = ScrolledWindow::new();
        scrolled.set_child(Some(&box_stack));
        scrolled.set_hexpand(true);
        scrolled.set_vexpand(true);

        let mut binding = get_counts_instance();
        let counts = binding.borrow_mut();

        box_home.append(&get_grid_values(
            counts.get_total(),
            counts.get_total_debt(),
            counts.get_total_perfomance(),
            counts.get_perfomance_months(),
        ));
        box_home.append(&box_button_lb);
        box_home.append(&scrolled);

        box_home
    }
    pub fn new_group_info(
        title: &str,
        nature: &str,
        infos: &Vec<Count>,
        stack: &Stack,
        stack_home: &Stack,
    ) -> Box {
        let mut soma: f32 = 0.0;
        let mut status: bool = true;

        for info in infos {
            soma += info.value;
        }

        for info in infos {
            if !info.status {
                status = false;
                break;
            }
        }

        let value_total = soma;

        let box_group = Box::new(Orientation::Horizontal, 0);
        box_group.add_css_class("box_group");
        box_group.set_hexpand(true);
        box_group.set_vexpand(true);

        box_group.set_valign(gtk::Align::Start);

        let box_left_g = Box::new(Orientation::Vertical, 2);
        box_left_g.add_css_class("box_left_g");
        box_left_g.set_hexpand(true);

        let value = format!("R${value_total:.2}");
        let value = Label::new(Some(&value));
        value.add_css_class("label_value_i");

        let title = Label::new(Some(title));
        title.add_css_class("name_i");

        let n_items = format!("{} itens", infos.len());
        let n_items = Label::new(Some(&n_items));
        n_items.add_css_class("name_i");

        box_left_g.append(&title);
        box_left_g.append(&value);
        box_left_g.append(&n_items);

        let box_right_g = Box::new(Orientation::Vertical, 2);
        box_right_g.add_css_class("box_right_g");

        let mut icon_path = PathBuf::from(format!("{}info_icon", var("ICON_PATH").unwrap()));
        icon_path.push(format!("{nature}.png"));

        if !icon_path.exists() {
            icon_path = PathBuf::from(format!(
                "{}info_icon/not_found.png",
                var("ICON_PATH").unwrap()
            ));
        }

        let icon = Image::from_file(icon_path);
        icon.add_css_class("icon_group");

        if status {
            icon.remove_css_class("negative");
            icon.add_css_class("positive");
        } else {
            icon.remove_css_class("positive");
            icon.add_css_class("negative");
        }

        let details = Button::with_label("Detalhes");
        details.set_css_classes(&["link_details"]);

        details.connect_clicked(clone!(
            #[weak]
            stack,
            #[strong]
            infos,
            #[weak]
            stack_home,
            move |_| {
                let _ = get_grid_infos(&stack, &stack_home, &infos, &title.text());
                stack.set_visible_child_name("details");
            }
        ));

        box_right_g.append(&icon);
        if !infos.is_empty() {
            box_right_g.append(&details);
        }

        box_group.append(&box_left_g);
        box_group.append(&box_right_g);
        box_group
    }
    pub fn new_debtor_info(debtor: &Debtor) -> Box {
        let box_debtors = Box::new(Orientation::Horizontal, 0);
        box_debtors.add_css_class("box_debtors");

        let grid: Grid = Grid::new();

        let name: Label = Label::new(Some(debtor.get_name()));

        let rece_value: Label = Label::new(Some(&format!("{:.2}", debtor.get_receipt())));
        let debt_value: Label = Label::new(Some(&format!("{:.2}", debtor.get_debt())));
        let paid_value: Label = Label::new(Some(&format!("{:.2}", debtor.get_value())));
        let total_value: Label = Label::new(Some(&format!(
            "{:.2}",
            debtor.get_value() + debtor.get_debt()
        )));
        let income_value: Label = Label::new(Some(&format!(
            "{:.2}",
            debtor.get_receipt() - (debtor.get_value() + debtor.get_debt())
        )));

        let rece_label: Label = Label::new(Some("Receita:"));
        let debt_label: Label = Label::new(Some("Devendo:"));
        let paid_label: Label = Label::new(Some("Pago:"));
        let total_label: Label = Label::new(Some("Total em Gastos:"));
        let income_label: Label = Label::new(Some("Rendimento de:"));
        rece_label.add_css_class("title_i");
        debt_label.add_css_class("title_i");
        paid_label.add_css_class("title_i");
        total_label.add_css_class("title_i");
        income_label.add_css_class("title_i");

        grid.attach(&name, 0, 0, 1, 1);

        grid.attach(&rece_label, 0, 1, 1, 1);
        grid.attach(&debt_label, 0, 2, 1, 1);
        grid.attach(&paid_label, 0, 3, 1, 1);
        grid.attach(&total_label, 0, 4, 1, 1);
        grid.attach(&income_label, 0, 5, 1, 1);

        grid.attach(&rece_value, 1, 1, 1, 1);
        grid.attach(&debt_value, 1, 2, 1, 1);
        grid.attach(&paid_value, 1, 3, 1, 1);
        grid.attach(&total_value, 1, 4, 1, 1);
        grid.attach(&income_value, 1, 5, 1, 1);

        box_debtors.append(&grid);
        box_debtors.set_hexpand(true);
        box_debtors.set_halign(gtk::Align::Fill);

        box_debtors
    }
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

                let mut is_new_people = false;
                let name = if name_input.active_text().unwrap() != "+ Novo devedor" {
                    name_input.active_text().unwrap().to_string()
                } else {
                    is_new_people = true;
                    new_name_input.text().to_string()
                };

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

                tracing::info!("count is empty? {}", count.is_empty());
                if !count.is_empty() {
                    use tokio::runtime::Runtime;
                    let rnt = Runtime::new().unwrap();

                    get_counts_instance().put(count);

                    match rnt.block_on(control::save()) {
                        Ok(_) => {
                            if is_new_people {
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

    pub fn get_pay_box(&self) -> Box {
        let box_pay = Box::new(Orientation::Vertical, 10);
        box_pay.set_halign(gtk::Align::Center);
        box_pay.set_hexpand(true);

        let box_title = Box::new(Orientation::Horizontal, 10);
        box_title.add_css_class("title_i");

        let button_return = Button::new();
        button_return.add_css_class("link_return");

        button_return.set_label("Retornar");
        button_return.connect_clicked(clone!(
            #[weak]
            &self.stack,
            move |_| {
                self.stack.set_visible_child_name("home");
            }
        ));

        box_title.append(&Label::new(Some("Pagando conta")));
        box_title.append(&button_return);

        box_pay.append(&box_title);

        let binding = get_counts_instance();
        let infos = binding.borrow();
        let grid = Grid::new();
        grid.set_halign(gtk::Align::Center);
        grid.set_column_spacing(10);
        grid.set_row_spacing(10);

        let mut x = 0;
        let mut y = 0;

        if !infos.list.is_empty() {
            for info in &infos.list {
                if !info.status {
                    let group = box_info(info, stack);
                    grid.attach(&group, x, y, 1, 1);

                    if x < 1 {
                        x += 1;
                    } else {
                        x = 0;
                        y += 1;
                    }
                }
            }
        } else {
            grid.attach(&Label::new(Some("Nenhuma conta para pagar :D")), 1, 1, 1, 1);
        }

        let scrolled = ScrolledWindow::new();
        scrolled.set_child(Some(&grid));
        scrolled.set_size_request(600, 598);

        box_pay.append(&scrolled);

        box_pay
    }
    pub fn new_month_info(
        title: &str,
        nature: &str,
        infos: &Vec<Count>,
        stack: &Stack,
        stack_home: &Stack,
    ) -> Box {
        let mut soma: f32 = 0.0;

        for info in infos {
            if info.nature.to_lowercase() == "receita" {
                soma += info.value;
            } else if info.nature.to_lowercase() != "investimentos" {
                soma -= info.value;
            }
        }

        let value_total = soma;

        let box_group = Box::new(Orientation::Horizontal, 20);
        box_group.add_css_class("box_group");
        box_group.set_hexpand(true);

        let box_left_g = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(2)
            .hexpand(true)
            .vexpand(true)
            .halign(gtk::Align::Center)
            .valign(gtk::Align::Center)
            .css_classes(["box_left_g"])
            .build();

        let value = format!("R${value_total:.2}");
        let value = Label::new(Some(&value));
        value.add_css_class("label_value_i");

        let title = Label::new(Some(title));
        title.add_css_class("name_i");

        let n_items = format!("{} itens", infos.len());
        let n_items = Label::new(Some(&n_items));
        n_items.add_css_class("name_i");

        box_left_g.append(&title);
        box_left_g.append(&value);
        box_left_g.append(&n_items);

        let box_right_g = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(2)
            .hexpand(true)
            .vexpand(true)
            .halign(gtk::Align::End)
            .valign(gtk::Align::Center)
            .css_classes(["box_right_g"])
            .build();

        let mut icon_path = PathBuf::from(format!("{}info_icon", var("ICON_PATH").unwrap()));
        icon_path.push(format!("{nature}.png"));

        if !icon_path.exists() {
            icon_path = PathBuf::from(format!(
                "{}info_icon/not_found.png",
                var("ICON_PATH").unwrap()
            ));
        }

        let icon = Image::from_file(icon_path);
        icon.add_css_class("icon_group");

        if soma >= 0.0 {
            icon.remove_css_class("negative");
            icon.add_css_class("positive");
        } else {
            icon.remove_css_class("positive");
            icon.add_css_class("negative");
        }

        let details = Button::with_label("Detalhes");
        details.set_css_classes(&["link_details"]);

        details.connect_clicked(clone!(
            #[weak]
            stack,
            #[strong]
            infos,
            #[weak]
            stack_home,
            move |_| {
                let _ = get_grid_infos(&stack, &stack_home, &infos, &title.text());
                stack.set_visible_child_name("details");
            }
        ));

        box_right_g.append(&icon);
        if !infos.is_empty() {
            box_right_g.append(&details);
        }

        box_group.append(&box_left_g);
        box_group.append(&box_right_g);

        box_group
    }
    pub fn right() -> Box {
        let box_right = Box::new(Orientation::Vertical, 20);
        box_right.set_hexpand(true);
        box_right.set_vexpand(true);
        box_right.set_halign(gtk::Align::Fill);
        box_right.add_css_class("box_right_bb");

        let history = Label::new(Some("Histórico"));

        let order_by = Label::new(Some("Ordernar por: "));
        let drop_order = DropDown::from_strings(&[
            "⤓ Entrada",
            "⤒ Entrada",
            "⤓ Data limite",
            "⤒ Data limite",
            "⤓ Valor",
            "⤒ Valor",
            "⤓ Natureza",
            "⤒ Natureza",
            "⤓ Status",
            "⤒ Status",
            "⤓ Devedor",
            "⤒ Devedor",
        ]);
        drop_order.set_css_classes(&["dropdown_order_by"]);

        let box_order = Box::new(Orientation::Horizontal, 0);
        box_order.append(&order_by);
        box_order.append(&drop_order);

        let box_head = Box::new(Orientation::Horizontal, 100);
        box_head.append(&history);
        box_head.append(&box_order);

        box_head.add_css_class("box_head_bbr");
        box_head.set_hexpand(true);
        box_head.set_halign(gtk::Align::Center);

        let scrolled = ScrolledWindow::new();
        scrolled.add_css_class("list_info_history");
        scrolled.set_vexpand(true);
        scrolled.set_hexpand(true);

        let box_list_count = get_list_box();
        box_list_count.add_css_class("list_info_history");
        box_list_count.set_vexpand(true);
        box_list_count.set_hexpand(true);

        let counts = get_counts_instance();

        drop_order.connect_selected_item_notify(move |drop_order| {
            let mut counts = get_counts_instance().clone();

            match drop_order.selected() {
                0 => counts.order_by_date(true, true),
                1 => counts.order_by_date(true, false),
                2 => counts.order_by_date(false, true),
                3 => counts.order_by_date(false, false),
                4 => counts.order_by_value(true),
                5 => counts.order_by_value(false),
                6 => counts.order_alphabetical("nature", true),
                7 => counts.order_alphabetical("nature", false),
                8 => counts.order_by_status(true),
                9 => counts.order_by_status(false),
                10 => counts.order_alphabetical("name", true),
                11 => counts.order_alphabetical("name", false),
                _ => {}
            }

            reload_home(Some(&counts), None);
            std::mem::drop(counts);
        });

        for count in &counts.list {
            box_list_count.append(&new_box_info(count));
        }

        std::mem::drop(counts);

        scrolled.set_child(Some(box_list_count));

        box_right.append(&box_head);
        box_right.append(&scrolled);
        box_right
    }

    pub fn left() -> Box {
        let box_left = Box::new(Orientation::Vertical, 10);
        box_left.add_css_class("box_left_bb");
        box_left.set_hexpand(true);
        box_left.set_vexpand(true);
        box_left.set_halign(gtk::Align::Fill);

        let stack_left = Stack::new();
        stack_left.add_titled(&get_home_box(&stack_left), Some("home"), "home");
        stack_left.add_titled(&get_add_box(&stack_left), Some("addition"), "addition");
        stack_left.add_titled(&get_pay_box(&stack_left), Some("payment"), "payment");
        stack_left.set_vexpand(true);
        stack_left.set_hexpand(true);

        box_left.append(&stack_left);
        box_left
    }
}

impl GridCount for BoxCount {
    fn get_grid_debtors() -> Grid {
        let grid_debtors = Grid::builder()
            .halign(gtk::Align::Fill)
            .column_homogeneous(true)
            .column_spacing(10)
            .hexpand(true)
            .vexpand(true)
            .build();

        let binding = get_counts_instance();
        let counts = binding.borrow();
        let debtors: Vec<Debtor> = counts.filter_debtors();

        let mut x = 0;
        let mut y = 0;

        for debtor in &debtors {
            grid_debtors.attach(&new_debtor_info(debtor), x, y, 1, 1);

            if x < 1 {
                x += 1;
            } else {
                x = 0;
                y += 1;
            }
        }

        grid_debtors
    }
    fn get_grid_groups(stack: &Stack, stack_home: &Stack) -> Grid {
        let grid_groups = Grid::builder()
            .halign(gtk::Align::Fill)
            .valign(gtk::Align::Start)
            .column_homogeneous(true)
            .column_spacing(10)
            .row_spacing(10)
            .hexpand(true)
            .vexpand(true)
            .build();

        let counts = get_counts_instance();

        let rnt = tokio::runtime::Runtime::new().unwrap();
        let natures = match rnt.block_on(control::get_groups()) {
            Ok(groups) => groups,
            Err(err) => {
                tracing::error!("{:?}", err);
                vec![
                    String::from("Casa"),
                    String::from("Transporte"),
                    String::from("Alimentação"),
                    String::from("Saúde"),
                    String::from("Lazer"),
                    String::from("Receita"),
                    String::from("Outros"),
                ]
            }
        };

        for (i, nature) in natures.iter().enumerate() {
            grid_groups.attach(
                &new_group_info(
                    nature,
                    &nature.to_lowercase(),
                    &counts.filter_by_nature(nature),
                    stack,
                    stack_home,
                ),
                (i % 2) as i32,
                (i - (i % 2)) as i32,
                1,
                1,
            );
        }

        grid_groups
    }
    fn get_grid_infos(stack: &Stack, stack_home: &Stack, infos: &Vec<Count>, title: &str) -> Grid {
        let grid_infos = Grid::builder()
            .halign(gtk::Align::Fill)
            .valign(gtk::Align::Start)
            .column_homogeneous(true)
            .column_spacing(10)
            .row_spacing(10)
            .hexpand(true)
            .build();

        let mut x = 0;
        let mut y = 0;

        for info in infos {
            let group = box_info(info, stack_home);
            grid_infos.attach(&group, x, y, 1, 1);

            if x < 1 {
                x += 1;
            } else {
                x = 0;
                y += 1;
            }
        }

        if let Some(child) = stack.child_by_name("details") {
            stack.remove(&child);
        }

        stack.add_titled(&grid_infos, Some("details"), title);
        grid_infos
    }
    fn get_grid_months(stack: &Stack, stack_home: &Stack) -> Grid {
        let grid_months = Grid::builder()
            .halign(gtk::Align::Fill)
            .column_homogeneous(true)
            .column_spacing(10)
            .row_spacing(10)
            .hexpand(true)
            .vexpand(true)
            .build();

        let counts = get_counts_instance();

        let mut months: Vec<(String, Vec<Count>)> = vec![
            (String::from("Janeiro"), Vec::new()),
            (String::from("Fevereiro"), Vec::new()),
            (String::from("Março"), Vec::new()),
            (String::from("Abril"), Vec::new()),
            (String::from("Maio"), Vec::new()),
            (String::from("Junho"), Vec::new()),
            (String::from("Julho"), Vec::new()),
            (String::from("Agosto"), Vec::new()),
            (String::from("Setembro"), Vec::new()),
            (String::from("Outubro"), Vec::new()),
            (String::from("Novembro"), Vec::new()),
            (String::from("Dezembro"), Vec::new()),
        ];

        months = counts.filter_by_month(months);

        for (i, month) in months.iter().enumerate() {
            grid_months.attach(
                &new_month_info(
                    &month.0,
                    &month.0.to_lowercase(),
                    &month.1,
                    stack,
                    stack_home,
                ),
                (i % 2) as i32,
                (i - (i % 2)) as i32,
                1,
                1,
            );
        }

        grid_months
    }
    fn get_grid_values(count: f32, debt: f32, paid: f32, month: Vec<f32>) -> Grid {
        let grid = Grid::builder()
            .halign(gtk::Align::Center)
            .column_homogeneous(true)
            .row_spacing(10)
            .hexpand(true)
            .build();

        let box_count = Box::new(Orientation::Vertical, 10);

        let title = Label::new(Some("Total movimentado no ano"));
        let text = Label::new(Some(&format!("R$ {count:.2}")));
        title.add_css_class("name_i");
        text.add_css_class("value_total");

        box_count.append(&title);
        box_count.append(&text);

        let box_debt = Box::new(Orientation::Vertical, 10);
        box_debt.set_halign(gtk::Align::Center);
        box_debt.set_valign(gtk::Align::Center);

        let title = Label::new(Some("Total em dívidas abertas"));
        let text = Label::new(Some(&format!("R$ -{debt:.2}")));
        title.add_css_class("name_i");
        text.add_css_class("value_total");
        text.add_css_class("status_negative");

        box_debt.append(&title);
        box_debt.append(&text);

        let box_paid = Box::new(Orientation::Vertical, 10);
        box_paid.set_halign(gtk::Align::Center);
        box_paid.set_valign(gtk::Align::Center);

        let title = Label::new(Some("Saldo total"));
        let text = Label::new(Some(&format!("R$ {paid:.2}")));
        title.add_css_class("name_i");
        text.add_css_class("value_total");
        if paid > 0.0 {
            text.add_css_class("status_positive");
        } else {
            text.add_css_class("status_negative");
        }

        box_paid.append(&title);
        box_paid.append(&text);

        let box_count_month = Box::new(Orientation::Vertical, 10);

        use chrono::Utc;
        let month_index = Utc::now();
        let months = [
            "Janeiro",
            "Fevereiro",
            "Março",
            "Abril",
            "Maio",
            "Junho",
            "Julho",
            "Agosto",
            "Setembro",
            "Outubro",
            "Novembro",
            "Dezembro",
        ];

        let title = Label::new(Some(&format!(
            "Perfomance em {}",
            months[month_index.month0() as usize]
        )));
        let text = Label::new(Some(&format!(
            "R$ {:.2}",
            month[month_index.month0() as usize]
        )));
        title.add_css_class("name_i");
        text.add_css_class("value_total");

        box_count_month.append(&title);
        box_count_month.append(&text);

        grid.attach(&box_count, 0, 0, 1, 1);
        grid.attach(&box_debt, 1, 0, 1, 1);
        grid.attach(&box_paid, 2, 0, 1, 1);

        grid.attach(&box_count_month, 1, 1, 1, 1);

        grid
    }
}
