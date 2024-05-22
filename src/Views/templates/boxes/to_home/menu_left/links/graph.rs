use super::*;
use crate::env;
#[allow(unused)]
pub fn box_graph_link(stack: &Stack) -> Box {
    let fbox_graph = Box::new(Orientation::Horizontal, 0);
    let graph_icon =
        Image::from_file(format!("{}graph.png", env::var("ICON_PATH").unwrap()).trim());
    let graph_link = Button::with_label("Gráficos");
    graph_link.set_css_classes(&["link_view"]);

    graph_link.connect_clicked(clone!(@weak stack, @weak graph_link => move |_| {
        stack.set_visible_child_name("Graficos");
    }));

    graph_icon.add_css_class("icon_ml");

    fbox_graph.append(&graph_icon);
    fbox_graph.append(&graph_link);
    fbox_graph.set_valign(gtk::Align::Center);

    fbox_graph
}
