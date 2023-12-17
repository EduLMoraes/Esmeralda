use dioxus_elements::br;

use crate::structs::Info;
use super::*;

#[component]
pub fn Home (cx: Scope) -> Element {
    let total_counts = use_state(cx, || 0.0);
    let counts: Vec<Info> = Vec::new();

    render!{
        link{ r#rel: "stylesheet", href: "./src/view/styles/home.css" }

        h2{ id: "name",  "Esmeralda" }

        

        div{ id: "div-body",
            div{ id: "div-table",
                table{ id: "table-counts", 
                    th{ format!("Contas: {}", total_counts) }
                    tr{
                        td{ "Devedor" },
                        td{ "Valor" },
                        td{ "Data Inicial" },
                        td{ "Data Final" },
                        td{ "Parcelas" },
                        td{hidden: true, "Status" }
                    }
                    for info in counts {
                        tr{
                            td{hidden: true, format!("{}", info.debtor) },
                            td{ format!("{:2}", info.value) },
                            td{ format!("{}", info.date_in) },
                            td{ format!("{}", info.date_out) },
                            td{ format!("{}", info.installments) },
                            td{ format!("{}", info.status) },
                        }
                    }
                }
            }

            div{ id: "div-optiions", 
                button{ "Adicionar" },
                button{ "Pagar" },
                button{ "Editar" },
                button{ "Exportar" },

                p{
                    label{
                        "Devedor:" br{}
                        input{ r#type: "text", id: "debtor" }
                    }

                    br{}
                    
                    label{
                        "Valor:" br{}
                        input{ r#type: "numeric", id: "value" }
                    }
                }

                p{
                    label{
                        "Data:" br{}
                        input{ r#type: "date", id: "date_in" }
                        " - até - "
                        input{ r#type: "date", id: "date_out" }
                    }

                }

                p{
                    label{
                        "Parcelas:"
                        input{  r#type: "number", id: "installments" }
                    }

                    br{}

                    label{
                        "Já tá pago?"
                        input{ r#type: "checkbox", id: "payment" }
                    }
                }
            }
        }
    }
}