use super::*;
use crate::prelude::controller::save_in_file;
use crate::tokio::runtime;


pub fn div_options(cx: Scope) -> Element{
    let hidden_add: &UseState<bool> = use_state(cx, || true);
    let hidden_paid: &UseState<bool> = use_state(cx, || true);
    let hidden_edit: &UseState<bool> = use_state(cx, || true);
    let hidden_export: &UseState<bool> = use_state(cx, || true);

    let is_confirm: &UseState<bool> = use_state(cx, || false);

    let title: &UseState<String> = use_state(cx, || String::new());

    let mut path: String = String::from("./esmeralda_exportados/");
    let path_export: &UseState<String> = use_state(cx, || String::new());
    let extend: &UseState<String> = use_state(cx, || String::from(".csv"));
    let file: &UseState<String> = use_state(cx, || String::new());

    let rnt = runtime::Runtime::new().unwrap();

    let counts = use_shared_state::<InterfaceInfo>(cx).unwrap().read().clone();

    render!(
        div{ id: "div-optiions",
            div{ id: "div-buttons",
                h3{ "Ações" }
                button{ onclick: move |_| {
                    is_confirm.set(false); 
                    hidden_export.set(true);
                    hidden_edit.set(true);
                    hidden_paid.set(true);
                    hidden_add.set(!**hidden_add);
                }, "Adicionar"},
                button{ onclick: move |_| {
                    is_confirm.set(false); 
                    hidden_export.set(true);
                    hidden_edit.set(true);
                    hidden_paid.set(!**hidden_paid);
                    hidden_add.set(true);
                }, "Pagar" },
                button{ onclick: move |_| {
                    is_confirm.set(false);
                    hidden_export.set(true);
                    hidden_edit.set(!**hidden_edit);
                    hidden_paid.set(true);
                    hidden_add.set(true);
                }, "Editar" },
                button{ onclick: move |_| {
                    is_confirm.set(false);
                    hidden_export.set(!**hidden_export);
                    hidden_edit.set(true);
                    hidden_paid.set(true);
                    hidden_add.set(true);
                }, "Exportar" },
            }
            
            div{ id: "div-form-buttons",
                hidden: **hidden_add, 

                h4{ "Adicionando "}
                form{
                    p{
                        label{
                            "Nome:" br{}
                            input{ r#type: "text", id: "debtor" }
                        }
                        
                        br{}

                        label{
                            "Título:" br{}
                            input{ r#type: "text", id: "title", oninput: move |ev| title.set(ev.value.to_string()) }
                        }
    
                        br{}
                        
                        label{
                            "Valor:" br{}
                            input{ r#type: "numeric", id: "value" }
                        }
                        
                        br{}

                        label{
                            "Data:" br{}
                            input{ r#type: "date", id: "date_in" }
                            " - até - "
                            input{ r#type: "date", id: "date_out" }
                        }
    
                        br{}

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

                    button{ id: "confirm-form",
                        onclick: move |_| {
                            is_confirm.set(true);
                        },
                        "Confirmar"
                    }

                    p{ hidden: !**is_confirm, "Conta {title} adicionada!" }
                }
            }
        
            div{ id: "div-form-buttons",
                hidden: **hidden_paid, 

                h4{ "Pagando" }
                form{
                    p{
                        label{ "Informe o ID da conta a ser paga: "}
                        br{}
                        input{ r#type: "number", r#min: "0"}
                    }

                    button{ id: "confirm-form",
                        onclick: move |_| {
                            is_confirm.set(true);
                        },
                        "Confirmar"
                    }

                    p{ hidden: !**is_confirm, "Conta Status de {title} ficou como paga!"}
                }
            }
        
            div{ id: "div-form-buttons",
                hidden: **hidden_edit, 

                h4{ "Editando conta "}
                form{
                    p{
                        label{ "ID:" } br{}
                        input{ r#type: "number", r#min: "0" }

                        br{}

                        label{ "Coluna:" } br{}
                        input{ r#type: "text", placeholder: "Ex.: Título" }

                        label{ "Novo valor:"} br{}
                        input{ r#type: "text", placeholder: "Ex.: Frango do Peruzzo" }
                    }

                    button{ id: "confirm-form",
                        onclick: move |_| {
                            is_confirm.set(true);
                        },
                        "Confirmar"
                    }

                    p{ hidden: !**is_confirm, "Conta {title} editada!"}
                }
            }
    
            div{ id: "div-form-buttons",
                hidden: **hidden_export, 

                h4{ "Exportando" }
                form{
                    p{
                        label{ "Nome do arquivo: "} br{}
                        input{
                            value: "{file}",
                            oninput: move |name| file.set(name.value.clone())
                        }

                        label{ "Formato: "} br{}
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
                            path.push_str(file.trim());
                            path.push_str(extend.trim());
                            path_export.set(path.clone());

                            rnt.block_on(save_in_file(path.trim(), counts.clone())).unwrap();

                            is_confirm.set(true);
                        },
                        "Confirmar"
                    }

                    p{ hidden: !**is_confirm, "Última exportação foi '{path_export}' "}
                }
            }

        }
    )
}