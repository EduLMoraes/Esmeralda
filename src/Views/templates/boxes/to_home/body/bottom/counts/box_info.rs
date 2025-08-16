use self::control::edit;
use super::*;
use crate::{
    gtk::{
        gdk::BUTTON_SECONDARY, GestureClick, PopoverMenu,
        ResponseType,
    },
    prelude::control::update_counts_with_db,
};
use alerts::{confirm, edit_count};

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
