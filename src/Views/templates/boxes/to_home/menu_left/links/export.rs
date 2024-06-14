use super::*;

#[allow(deprecated)]
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
                            // alert("Erro ao exportar arquivo", "Erro")
                        }
                    }
                    None => {} //alert("Nenhum caminho escolhido!", "Erro: caminho inválido"),
                }
            }

            window.close()
        });

        fc.show();
    });

    count_icon.add_css_class("icon_ml");

    fbox_count.append(&count_icon);
    fbox_count.append(&count_link);
    fbox_count.set_valign(gtk::Align::Center);

    fbox_count
}
