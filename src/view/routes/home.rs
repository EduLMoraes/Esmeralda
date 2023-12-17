mod div_options;
mod div_active;
use dioxus_elements::GlobalAttributes;

use crate::structs::Info;
use super::*;

#[derive(Clone, Debug)]
struct Columns{
    name: bool,
    title: bool,
    description: bool,
    date_in: bool,
    date_out: bool,
    paid_installments: bool,
    installments: bool,
    value: bool,
    status: bool
}

impl Columns{
    pub fn new() -> Columns{
        Columns{
            name: false,
            title: false,
            description: false,
            date_in: false,
            date_out: false,
            paid_installments: false,
            installments: false,
            value: false,
            status: false
        }
    }
}

#[component]
pub fn Home (cx: Scope) -> Element {
    let total_counts = use_state(cx, || 0.0);
    let counts: Vec<Info> = Vec::new();

    use_shared_state_provider(cx, || Columns::new());
    let columns = use_shared_state::<Columns>(cx).unwrap().read().clone();

    render!{

        link{ r#rel: "stylesheet", href: "./src/view/styles/home.css" }

        h2{ id: "name",  "Esmeralda" }

        div{ id: "div-body",
            div_active::div_most(cx),

            div{ id: "div-table",
                table{ id: "table-counts", 
                    th{ format!("Contas: {}", total_counts) }
                    tr{
                        td{hidden: !columns.name, "Nome" },
                        td{hidden: !columns.title, "Título" },
                        td{hidden: !columns.description, "Descrição" },
                        td{hidden: !columns.date_in, "Data Inicial" },
                        td{hidden: !columns.date_out, "Data Final" },
                        td{hidden: !columns.paid_installments, "Parcelas Pagas" },
                        td{hidden: !columns.installments, "Parcelas" },
                        td{hidden: !columns.value, "Valor" },
                        td{hidden: !columns.status, "Status" }
                    }
                    for info in counts {
                        tr{
                            td{hidden: !columns.name, format!("{}", info.debtor) },
                            td{hidden: !columns.title, format!("{}", info.title) },
                            td{hidden: !columns.description, format!("{}", info.description) },
                            td{hidden: !columns.date_in, format!("{}", info.date_in) },
                            td{hidden: !columns.date_out, format!("{}", info.date_out) },
                            td{hidden: !columns.paid_installments, format!("{}", info.paid_installments) },
                            td{hidden: !columns.installments, format!("{}", info.installments) },
                            td{hidden: !columns.value, format!("{:2}", info.value) },
                            td{hidden: !columns.status, format!("{}", info.status) },
                        }
                    }
                }
            }
            div_options::div_options(cx)
        }
    }
}