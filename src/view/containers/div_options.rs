use super::*;
use crate::prelude::structs::Message;

mod div_add;
mod div_edit;
mod div_export;
mod div_paid;

/// Renders a div element with buttons for different actions and includes other modules based on the value of the hidden flags.
///
/// # Arguments
///
/// * `cx` - The scope parameter used to create the HTML element.
///
/// # Example
///
/// ```rust
/// div_options(scope);
/// ```
pub fn div_options(cx: Scope) -> Element {
    use_shared_state_provider::<Message>(cx, || Message {
        hidden: true,
        text: "",
    });

    let msg = use_shared_state::<Message>(cx).unwrap();

    let hidden_add: &UseState<bool> = use_state(cx, || true);
    let hidden_paid: &UseState<bool> = use_state(cx, || true);
    let hidden_edit: &UseState<bool> = use_state(cx, || true);
    let hidden_export: &UseState<bool> = use_state(cx, || true);

    render!(
        div{ id: "div-optiions",
            div{ id: "div-buttons",
                h3{ "Ações" }
                button{ onclick: move |_| {
                    msg.write().hidden = true;
                    hidden_export.set(true);
                    hidden_edit.set(true);
                    hidden_paid.set(true);
                    hidden_add.set(!**hidden_add);
                }, "Adicionar"},
                button{ onclick: move |_| {
                    msg.write().hidden = true;
                    hidden_export.set(true);
                    hidden_edit.set(true);
                    hidden_paid.set(!**hidden_paid);
                    hidden_add.set(true);
                }, "Pagar" },
                button{ onclick: move |_| {
                    msg.write().hidden = true;
                    hidden_export.set(true);
                    hidden_edit.set(!**hidden_edit);
                    hidden_paid.set(true);
                    hidden_add.set(true);
                }, "Editar" },
                button{ onclick: move |_| {
                    msg.write().hidden = true;
                    hidden_export.set(!**hidden_export);
                    hidden_edit.set(true);
                    hidden_paid.set(true);
                    hidden_add.set(true);
                }, "Exportar" },
            }

            div_add::add(cx, *hidden_add.get())

            div_paid::paid(cx, *hidden_paid.get())

            div_edit::edit(cx, *hidden_edit.get())

            div_export::export(cx, *hidden_export.get())

        }
    )
}
