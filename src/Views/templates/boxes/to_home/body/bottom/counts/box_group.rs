use super::*;

pub fn new_group_info(title: &str, nature: &str, infos: &Vec<Count>, stack: &Stack, stack_home: &Stack) -> Box {
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

    let box_left_g = Box::new(Orientation::Vertical, 2);
    box_left_g.add_css_class("box_left_g");
    box_left_g.set_valign(gtk::Align::Center);
    box_left_g.set_halign(gtk::Align::Start);

    let value = format!("R${:.2}", value_total);
    let value = Label::new(Some(&value));
    value.add_css_class("label_value_i");

    let title = Label::new(Some(title));
    title.add_css_class("name_i");

    let n_items = format!("{} itens", infos.len());
    let n_items = Label::new(Some(&n_items));
    n_items.add_css_class("name_i");

    box_left_g.append(&value);
    box_left_g.append(&title);
    box_left_g.append(&n_items);

    let box_right_g = Box::new(Orientation::Vertical, 2);
    box_right_g.add_css_class("box_right_g");
    box_right_g.set_valign(gtk::Align::Center);
    box_right_g.set_halign(gtk::Align::Center);

    let mut icon_path = PathBuf::from(format!("{}info_icon", var("ICON_PATH").unwrap()));
    icon_path.push(format!("{}.png", nature));

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

    details.connect_clicked(clone!(@strong stack, @strong infos, @strong stack_home => move |_| {
        let _ = get_grid_infos(&stack, &stack_home, &infos);
        stack.set_visible_child_name(&title.label());
    }));

    box_right_g.append(&icon);
    if !infos.is_empty() {
        box_right_g.append(&details);
    }

    box_group.append(&box_left_g);
    box_group.append(&box_right_g);

    box_group
}
