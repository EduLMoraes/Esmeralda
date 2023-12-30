use super::*;

use crate::prelude::structs::Debtor;

#[path = "../../controller/utils/filter.rs"]
mod filter;

/// Generates an HTML element representing a table with checkboxes for each column and a list of debtors.
///
/// # Arguments
///
/// * `cx` - The scope object used to access shared state.
///
/// # Example
///
/// ```rust
/// let cx: Scope = ...; // create a scope
/// let element: Element = div_most(cx); // generate the HTML element
/// ```
///
/// # Returns
///
/// The generated HTML element representing the table.
///
pub fn div_most(cx: Scope) -> Element {
    let columns: &UseSharedState<Columns> = use_shared_state::<Columns>(cx).unwrap();
    let col_now: Columns = columns.read().clone();

    let counts: &UseSharedState<InterfaceInfo> = use_shared_state::<InterfaceInfo>(cx).unwrap();
    let list_counts = counts.read().order_alphabetical("debtor", true);
    let debtors: Vec<Debtor> = filter::filter_debtors(list_counts.list);

    render!(
        div{ id: "div-most",
            table{
                tr{
                    td{ "Coluna" },
                    td{ "Ativar" }
                }

                tr{
                    td{id: "td-most", "Nome" },
                    td{ onclick: move |_| columns.write().name = !col_now.name,
                        input{
                            r#type: "checkbox",
                            id: "most",
                            checked: col_now.name,
                        }
                    }
                }

                tr{
                    td{id: "td-most", "Título" },
                    td{ onclick: move |_| columns.write().title = !col_now.title, 
                        input{
                            r#type: "checkbox",
                            id: "most",
                            checked: col_now.title,
                            onclick: move |_| columns.write().title = !col_now.title
                        }
                    }
                }

                tr{
                    td{id: "td-most", "Descrição" },
                    td{ onclick: move |_| columns.write().description = !col_now.description,
                        input{
                            r#type: "checkbox",
                            id: "most",
                            checked: col_now.description,
                        }
                    }
                }

                tr{
                    td{id: "td-most", "Datas de entrada" },
                    td{ onclick: move |_| columns.write().date_in = !col_now.date_in,
                        input{
                            r#type: "checkbox",
                            id: "most",
                            checked: col_now.date_in,
                        }
                    }
                }

                tr{
                    td{id: "td-most", "Datas de saída" },
                    td{ onclick: move |_| columns.write().date_out = !col_now.date_out,
                        input{
                            r#type: "checkbox",
                            id: "most",
                            checked: col_now.date_out,
                        }
                    }
                }

                tr{
                    td{id: "td-most", "Parcelas pagas" },
                    td{ onclick: move |_| columns.write().paid_installments = !col_now.paid_installments,
                        input{
                            r#type: "checkbox",
                            id: "most",
                            checked: col_now.paid_installments,
                        }
                    }
                }

                tr{
                    td{id: "td-most", "Parcelas" },
                    td{ onclick: move |_| columns.write().installments = !col_now.installments,
                        input{
                            r#type: "checkbox",
                            id: "most",
                            checked: col_now.installments,
                        }
                    }
                }

                tr{
                    td{id: "td-most", "Valor" },
                    td{ onclick: move |_| columns.write().value = !col_now.value,
                        input{
                            r#type: "checkbox",
                            id: "most",
                            checked: col_now.value,
                        }
                    }
                }

                tr{
                    td{id: "td-most", "Status" },
                    td{ onclick: move |_| columns.write().status = !col_now.status,
                        input{
                            r#type: "checkbox",
                            id: "most",
                            checked: col_now.status,
                        }
                    }
                }

            }

            table{ id: "table-debtors",
                tr{
                    td { "Nome:" }
                    td { "Valor Dívida:"}
                    td { "Total:" }
                    td { "Status:" }
                }

                for debtor in debtors{
                    tr{
                        td { debtor.get_name().clone() }
                        td { format!("{:.2}", debtor.get_debt()) }
                        td { format!("{:.2}", debtor.get_value()) }
                        td { id: if debtor.get_status() { "stt-pos" } else { "stt-neg" } }
                    }
                }
            }
        }

    )
}
