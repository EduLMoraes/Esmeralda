use super::*;
use gtk::Image;

mod ml_body;
mod ml_head;

use ml_body::ml_body;
use ml_head::ml_head;

pub struct LeftMenu {}

impl LeftMenut {
    pub fn box_count_link(stack: &Stack) -> Box {
        let fbox_count = Box::new(Orientation::Horizontal, 0);
        let count_icon = Image::from_file(format!("{}counts.png", var("ICON_PATH").unwrap()));
        let count_link = Button::with_label("Contas");
        count_link.set_css_classes(&["link_view"]);

        count_link.connect_clicked(clone!(
            #[weak]
            stack,
            move |_| {
                reload_home(None, Some(&stack));
                stack.set_visible_child_name("Contas");
            }
        ));

        count_icon.add_css_class("icon_ml");

        fbox_count.append(&count_icon);
        fbox_count.append(&count_link);
        fbox_count.set_valign(gtk::Align::Center);

        fbox_count
    }
    pub fn box_export_link() -> Box {
        let fbox_count = Box::new(Orientation::Horizontal, 0);
        let count_icon = Image::from_file(format!("{}export.png", var("ICON_PATH").unwrap()));
        let count_link = Button::with_label("Exportar");
        count_link.set_css_classes(&["link_view"]);

        count_link.connect_clicked(|_| {
            use crate::gtk::{FileChooserDialog, ResponseType};

            let fc = FileChooserDialog::builder()
                .name("Exportar contas para...")
                .action(gtk::FileChooserAction::SelectFolder)
                .build();

            fc.add_buttons(&[
                ("Cancel", gtk::ResponseType::Cancel),
                ("Open", gtk::ResponseType::Accept),
            ]);

            fc.connect_response(|window, response| {
                if response == ResponseType::Accept {
                    match window.current_folder() {
                        Some(folder) => {
                            use crate::control::save_in_file;
                            use crate::tokio::runtime::Runtime;

                            let rnt = Runtime::new().unwrap();

                            let res = rnt.block_on(save_in_file(
                                &format!(
                                    "{}/Resumo_Esmeralda.csv",
                                    folder.path().unwrap().to_str().unwrap()
                                ),
                                get_counts_instance().borrow(),
                            ));

                            if res.is_err() {
                                alert("Erro ao exportar arquivo", "Erro")
                            } else {
                                alert("Sucesso ao exportar arquivo", "Concluído")
                            }
                        }
                        None => alert("Nenhum caminho escolhido!", "Erro: caminho inválido"),
                    }
                }

                window.destroy()
            });

            fc.show();
        });

        count_icon.add_css_class("icon_ml");

        fbox_count.append(&count_icon);
        fbox_count.append(&count_link);
        fbox_count.set_valign(gtk::Align::Center);

        fbox_count
    }
    pub fn box_graph_link(stack: &Stack) -> Box {
        let fbox_graph = Box::new(Orientation::Horizontal, 0);
        let graph_icon =
            Image::from_file(format!("{}graph.png", env::var("ICON_PATH").unwrap()).trim());
        let graph_link = Button::with_label("Gráficos");
        graph_link.set_css_classes(&["link_view"]);

        graph_link.connect_clicked(clone!(
            #[weak]
            stack,
            #[weak]
            graph_link,
            move |_| {
                reload_home(None, Some(&stack));
                stack.set_visible_child_name("Graficos");
            }
        ));

        graph_icon.add_css_class("icon_ml");

        fbox_graph.append(&graph_icon);
        fbox_graph.append(&graph_link);
        fbox_graph.set_valign(gtk::Align::Center);

        fbox_graph
    }
    pub fn box_investments_link(stack: &Stack) -> Box {
        let fbox_count = Box::new(Orientation::Horizontal, 0);
        let icon = Image::from_file(format!("{}investments.png", var("ICON_PATH").unwrap()));
        let link = Button::with_label("Investimentos");
        link.set_css_classes(&["link_view"]);

        link.connect_clicked(clone!(
            #[weak]
            stack,
            move |_| {
                reload_home(None, Some(&stack));
                stack.set_visible_child_name("Investimentos");
            }
        ));

        icon.add_css_class("icon_ml");

        fbox_count.append(&icon);
        fbox_count.append(&link);
        fbox_count.set_valign(gtk::Align::Center);

        fbox_count
    }
    pub fn links(stack: &Stack) -> Box {
        let box_links = Box::new(Orientation::Vertical, 20);

        box_links.append(&box_count_link(stack));
        box_links.append(&box_graph_link(stack));
        box_links.append(&box_investments_link(stack));
        box_links.append(&box_export_link());

        box_links.add_css_class("box_links_ml");
        box_links
    }
    pub fn get_box_menu_left(stack: &Stack) -> Box {
        let box_ml = Box::new(Orientation::Vertical, 0);

        box_ml.append(&ml_head());
        box_ml.append(&ml_body(stack));
        box_ml.add_css_class("box_ml");

        box_ml
    }
    pub fn ml_body(stack: &Stack) -> Box {
        let box_body = Box::new(Orientation::Vertical, 5);
        box_body.add_css_class("box_body_ml");

        box_body.append(&links(stack));
        box_body.set_valign(gtk::Align::Center);

        box_body
    }
    pub fn ml_head() -> Box {
        let box_head = Box::new(Orientation::Vertical, 0);
        box_head.add_css_class("box_head_ml");
        let version = Label::new(Some(&format!(
            "Versão {}",
            std::env::var("CARGO_PKG_VERSION").unwrap()
        )));
        version.add_css_class("version");

        let box_title = Box::new(Orientation::Horizontal, 0);
        box_title.add_css_class("box_title_ml");

        let icon = Image::from_file(format!("{}icon.png", var("ICON_PATH").unwrap()));
        icon.add_css_class("icon_ml");

        let title = Label::new(Some("Esmeralda"));
        title.add_css_class("title_ml");

        box_title.append(&icon);
        box_title.append(&title);
        box_title.append(&version);

        box_title.set_halign(gtk::Align::Center);

        let menu = Label::new(Some("Menu"));
        menu.add_css_class("section_ml");

        box_head.append(&box_title);
        box_head.append(&menu);

        box_head
    }
}
