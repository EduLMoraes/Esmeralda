#![allow(deprecated)]
use crate::{
    controller::save_in_file,
    model::List::get_counts_instance,
    views::{
        alerts::alert,
        view_home::{box_count::BoxCount, box_investment::Investments, box_plot::Plot, HomeView},
    },
};
use gtk::{Box, Button, FileChooserDialog, Image, Label, Orientation, ResponseType};
use std::env::var;
use tokio::runtime::Runtime;

pub struct LeftMenu<'a, T> {
    home: HomeView<'a, T>,
}
impl<'a, T> LeftMenu<'a, T> {
    pub fn new(home: HomeView<'a, T>) -> Self {
        Self { home }
    }

    pub fn get_box(&self) -> Box {
        let box_ml = Box::new(Orientation::Vertical, 0);

        box_ml.append(&self.box_head());
        box_ml.append(&self.box_body());
        box_ml.add_css_class("box_ml");

        box_ml
    }

    pub fn box_body(&self) -> Box {
        let box_body = Box::new(Orientation::Vertical, 5);
        box_body.add_css_class("box_body_ml");

        box_body.append(&self.links());
        box_body.set_valign(gtk::Align::Center);

        box_body
    }

    pub fn box_head(&self) -> Box {
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

    pub fn links(&self) -> Box {
        let box_links = Box::new(Orientation::Vertical, 20);

        box_links.append(&self.box_count_link());
        box_links.append(&self.box_graph_link());
        box_links.append(&self.box_investments_link());
        box_links.append(&self.box_export_link());

        box_links.add_css_class("box_links_ml");
        box_links
    }

    pub fn box_count_link(&self) -> Box {
        let option_box = Box::new(Orientation::Horizontal, 0);
        let icon = Image::from_file(format!("{}counts.png", var("ICON_PATH").unwrap()));
        let link = Button::with_label("Contas");
        link.set_css_classes(&["link_view"]);

        link.connect_clicked(move |_| {
            self.home.load_page(BoxCount::new());
        });

        icon.add_css_class("icon_ml");

        option_box.append(&icon);
        option_box.append(&link);
        option_box.set_valign(gtk::Align::Center);

        option_box
    }

    pub fn box_plot_link(&self) -> Box {
        let option_box = Box::new(Orientation::Horizontal, 0);
        let icon = Image::from_file(format!("{}graph.png", var("ICON_PATH").unwrap()));
        let link = Button::with_label("Gráficos");
        link.set_css_classes(&["link_view"]);

        link.connect_clicked(move |_| {
            self.home.load_page(Plot::new());
        });

        icon.add_css_class("icon_ml");

        option_box.append(&icon);
        option_box.append(&link);
        option_box.set_valign(gtk::Align::Center);

        option_box
    }

    pub fn box_investments_link(&self) -> Box {
        let option_box = Box::new(Orientation::Horizontal, 0);
        let icon = Image::from_file(format!("{}investments.png", var("ICON_PATH").unwrap()));
        let link = Button::with_label("Investimentos");
        link.set_css_classes(&["link_view"]);

        link.connect_clicked(move |_| {
            self.home.load_page(Investments::new());
        });

        icon.add_css_class("icon_ml");

        option_box.append(&icon);
        option_box.append(&link);
        option_box.set_valign(gtk::Align::Center);

        option_box
    }

    pub fn box_export_link(&self) -> Box {
        let fbox_count = Box::new(Orientation::Horizontal, 0);
        let count_icon = Image::from_file(format!("{}export.png", var("ICON_PATH").unwrap()));
        let count_link = Button::with_label("Exportar");
        count_link.set_css_classes(&["link_view"]);

        count_link.connect_clicked(|_| {
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
                            }
                            else {
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
}
