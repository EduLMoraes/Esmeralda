use super::*;

#[allow(deprecated)]
pub fn box_import_link() -> Box {
    let fbox_count = Box::new(Orientation::Horizontal, 0);
    let count_icon = Image::from_file(format!("{}import.png", var("ICON_PATH").unwrap()));
    let count_link = Button::with_label("Importar");
    count_link.set_css_classes(&["link_view"]);

    count_link.connect_clicked(|_| {
        use crate::gtk::{FileChooserDialog, ResponseType};

        let fc = FileChooserDialog::builder()
            .name("Importar contas de...")
            .action(gtk::FileChooserAction::Open)
            .build();

        fc.add_buttons(&[
            ("Cancel", gtk::ResponseType::Cancel),
            ("Open", gtk::ResponseType::Accept),
        ]);
        fc.connect_response(|window, response| {
            if response == ResponseType::Accept {
                match window.file() {
                    Some(file) => {
                        use crate::control::read_of_file;
                        use crate::tokio::runtime::Runtime;

                        let rnt = Runtime::new().unwrap();
                        let Some(path) = file.path() else {
                            alert("Nenhum caminho escolhido!", "Erro: caminho inválido");
                            return;
                        };
                        let Some(path) = path.to_str() else {
                            return;
                        };

                        let res = rnt.block_on(read_of_file(
                            &format!("{}", path),
                            get_counts_instance().borrow_mut(),
                        ));

                        if res.is_err() {
                            alert("Erro ao importar arquivo", "Erro")
                        } else {
                            alert("Sucesso ao importar arquivo", "Concluído")
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
