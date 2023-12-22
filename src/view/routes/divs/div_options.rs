use super::*;

mod div_add;
mod div_paid;
mod div_edit;
mod div_export;

struct Message<'a>{
    hidden: bool,
    text: &'a str
}

pub fn div_options(cx: Scope) -> Element{
    use_shared_state_provider::<Message>(cx, || Message{ hidden: true, text: ""});

    let hidden_add: &UseState<bool> = use_state(cx, || true);
    let hidden_paid: &UseState<bool> = use_state(cx, || true);
    let hidden_edit: &UseState<bool> = use_state(cx, || true);
    let hidden_export: &UseState<bool> = use_state(cx, || true);

    render!(
        div{ id: "div-optiions",
            div{ id: "div-buttons",
                h3{ "Ações" }
                button{ onclick: move |_| {
                    hidden_export.set(true);
                    hidden_edit.set(true);
                    hidden_paid.set(true);
                    hidden_add.set(!**hidden_add);
                }, "Adicionar"},
                button{ onclick: move |_| {
                    hidden_export.set(true);
                    hidden_edit.set(true);
                    hidden_paid.set(!**hidden_paid);
                    hidden_add.set(true);
                }, "Pagar" },
                button{ onclick: move |_| {
                    hidden_export.set(true);
                    hidden_edit.set(!**hidden_edit);
                    hidden_paid.set(true);
                    hidden_add.set(true);
                }, "Editar" },
                button{ onclick: move |_| {
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