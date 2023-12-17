use super::*;

pub fn div_options(cx: Scope) -> Element{
    render!(
        div{ id: "div-optiions",
            div{ id: "div-buttons",
                button{ "Adicionar" },
                button{ "Pagar" },
                button{ "Editar" },
                button{ "Exportar" },
            }
            
            div{ id: "div-form-buttons", 
                p{
                    label{
                        "Nome:" br{}
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
    )
}