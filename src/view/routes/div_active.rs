use super::*;

pub fn div_most(cx: Scope) -> Element{
    let columns = use_shared_state::<Columns>(cx).unwrap();
    let col_now = columns.read().clone();

    render!(
        div{ id: "div-most",
            table{
                tr{
                    td{ "Coluna" },
                    td{ "Ativar" }
                }

                tr{ 
                    td{id: "td-most", "Nome" },
                    td{ input{ r#type: "checkbox", id: "most", onclick: move |_| columns.write().name = !col_now.name}}
                }

                tr{ 
                    td{id: "td-most", "Título" },
                    td{ input{ r#type: "checkbox", id: "most", onclick: move |_| columns.write().title = !col_now.title }}
                }

                tr{ 
                    td{id: "td-most", "Descrição" },
                    td{ input{ r#type: "checkbox", id: "most", onclick: move |_| columns.write().description = !col_now.description }}
                }

                tr{ 
                    td{id: "td-most", "Datas de entrada" },
                    td{ input{ r#type: "checkbox", id: "most", onclick: move |_| columns.write().date_in = !col_now.date_in }}
                }

                tr{ 
                    td{id: "td-most", "Datas de saída" },
                    td{ input{ r#type: "checkbox", id: "most", onclick: move |_| columns.write().date_out = !col_now.date_out }}
                }

                tr{ 
                    td{id: "td-most", "Parcelas pagas" },
                    td{ input{ r#type: "checkbox", id: "most", onclick: move |_| columns.write().paid_installments = !col_now.paid_installments }}
                }

                tr{ 
                    td{id: "td-most", "Parcelas" },
                    td{ input{ r#type: "checkbox", id: "most", onclick: move |_| columns.write().installments = !col_now.installments }}
                }

                tr{ 
                    td{id: "td-most", "Valor" },
                    td{ input{ r#type: "checkbox", id: "most", onclick: move |_| columns.write().value = !col_now.value }}
                }

                tr{ 
                    td{id: "td-most", "Status" },
                    td{ input{ r#type: "checkbox", id: "most", onclick: move |_| columns.write().status = !col_now.status }}
                }

            }
        }

    )
}