use super::*;
use crate::prelude::control::save_in_file;
use crate::prelude::tokio::runtime;
use nfd::Response;

/// Renders a form that allows the user to select a file format for exporting data and handles the logic for saving the exported data to a file.
///
/// # Arguments
///
/// * `cx` - The scope of the component.
/// * `hidden_export` - A boolean value indicating whether the export form should be hidden or not.
///
/// # Example
///
/// ```rust
/// let cx = Scope::new();
/// let hidden_export = false;
/// export(cx, hidden_export);
/// ```
pub fn export(cx: Scope, hidden_export: bool) -> Element {
    let msg = use_shared_state::<Message>(cx).unwrap();

    let mut path: String = String::new();
    let path_export: &UseState<String> = use_state(cx, || String::new());
    let extend: &UseState<String> = use_state(cx, || String::from(".csv"));
    let counts: &UseSharedState<InterfaceInfo> = use_shared_state::<InterfaceInfo>(cx).unwrap();

    render!(
        div{ id: "div-form-buttons",
            hidden: hidden_export,

            h4{ "Exportando" }
            form{
                p{
                    label{ "Formato: "}
                    select{
                        onchange: move |option|{
                            extend.set(option.value.to_string());
                        },

                        option{ value: ".csv", onchange: move |option|{
                            extend.set(option.value.to_string());
                        }, ".csv"}
                        option{ value: ".pdf", onchange: move |option|{
                            extend.set(option.value.to_string());
                        }, ".pdf" }
                        option{ value: ".html", onchange: move |option|{
                            extend.set(option.value.to_string());
                        }, ".html" }
                    }
                }

                button{ id: "confirm-form",
                    onclick: move |_| {
                        let result = nfd::open_save_dialog(Some(extend.trim()), Some(extend.trim())).unwrap();

                        match result {
                            Response::Okay(file_path) => path.push_str(file_path.trim()),
                            Response::OkayMultiple(_files) => unimplemented!(),
                            Response::Cancel => {
                                msg.write().hidden = false;
                                msg.write().text = "Exportação cancelada!";
                                return;
                            },
                        }

                        path.push_str(extend.trim());
                        path_export.set(path.clone());

                        let rnt = runtime::Runtime::new().unwrap();
                        path_export.set(rnt.block_on(save_in_file(path.trim(), &counts.read().clone())).unwrap());

                        msg.write().hidden = false;
                        msg.write().text = "Exportação concluída!";
                    },
                    "Confirmar"
                }

                p{ hidden: msg.read().hidden, msg.read().text }
            }
        }
    )
}
