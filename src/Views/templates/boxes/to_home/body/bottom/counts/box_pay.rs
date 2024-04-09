use super::*;

pub fn get_pay_box(stack: &Stack) -> Box {
    let box_pay = Box::new(Orientation::Vertical, 10);
    box_pay.set_halign(gtk::Align::Center);

    let box_title = Box::new(Orientation::Horizontal, 10);
    box_title.add_css_class("title_i");

    let button_return = Button::new();
    button_return.add_css_class("link_return");

    button_return.set_label("Retornar");
    button_return.connect_clicked(clone!(@strong stack => move |_| {
        stack.set_visible_child_name("home");
    }));

    box_title.append(&Label::new(Some("Pagando conta")));
    box_title.append(&button_return);

    box_pay.append(&box_title);

    let infos = unsafe { GLOBAL_COUNTS.get().unwrap() };
    let grid = Grid::new();
    grid.set_halign(gtk::Align::Center);
    grid.set_column_spacing(10);
    grid.set_row_spacing(10);

    let mut x = 0;
    let mut y = 0;

    if !infos.list.is_empty() {
        for mut info in &infos.list {
            if !info.status {
                let group = box_info(&mut info, Some(stack));
                grid.attach(&group, x, y as i32, 1, 1);

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
