use super::*;

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
    let debtors = list_counts.filter_debtors();

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
                    td{id: "td-most", "Natureza do gasto" },
                    td{ onclick: move |_| columns.write().nature = !col_now.nature,
                        input{
                            r#type: "checkbox",
                            id: "most",
                            checked: col_now.nature,
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
                    td { id: "col-debtor-value", "Valor da Dívida:"}
                    td { id: "col-debtor-value", "Valor Pago Total:" }
                    td { "Status:" }
                }

                for debtor in debtors{

                    tr{
                        td { id: "col-name", debtor.get_name().clone() }
                        td { id: "col-value", format!("{:.2}", debtor.get_debt()) }
                        td { id: "col-value", format!("{:.2}", debtor.get_value()) }
                        td { div{ id: if debtor.get_status() { "stt-pos" } else { "stt-neg" } } }
                    }
                }
            }
        }

    )
}
