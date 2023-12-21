use chrono::NaiveDate;

use super::*;
use crate::prelude::controller::save_in_file;
use crate::prelude::controller::is_complete;
use crate::prelude::controller::is_alphabetic;
use crate::tokio::runtime;


pub fn div_options(cx: Scope) -> Element{
    let hidden_add: &UseState<bool> = use_state(cx, || true);
    let hidden_paid: &UseState<bool> = use_state(cx, || true);
    let hidden_edit: &UseState<bool> = use_state(cx, || true);
    let hidden_export: &UseState<bool> = use_state(cx, || true);

    let is_confirm: &UseState<bool> = use_state(cx, || false);

    let is_value_valid: &UseState<bool> = use_state(cx, || true);
    let is_inst_valid: &UseState<bool> = use_state(cx, || true);
    let is_name_valid: &UseState<bool> = use_state(cx, || true);
    let is_id_valid: &UseState<bool> = use_state(cx, || true);

    let mut path: String = String::from("./esmeralda_exportados/");
    let path_export: &UseState<String> = use_state(cx, || String::new());
    let extend: &UseState<String> = use_state(cx, || String::from(".csv"));
    let file: &UseState<String> = use_state(cx, || String::new());

    let counts = use_shared_state::<InterfaceInfo>(cx).unwrap();
    let info = use_state::<Info>(cx, || Info::new());

    let id_search =  use_state(cx, || 0);
    let mut has_count: bool = false;

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
                            input{ r#required: true, r#type: "text", id: "debtor", oninput: move |name| {
                                let name = name.value.clone();

                                is_name_valid.set(is_alphabetic(&name));

                                if **is_name_valid{
                                    let mut tmp_info = info.get().clone();
                                    tmp_info.debtor = name;
                                    info.set(tmp_info);
                                }

                                is_confirm.set(false);
                            } }
                        }
                        
                        p{ id: "data-invalid", hidden: **is_name_valid, "Nome inválido!" }

                        br{}

                        label{
                            "Título:" br{}
                            input{ r#required:true, r#type: "text", id: "title", oninput: move |title| {
                                let mut tmp_info = info.get().clone();
                                tmp_info.title = title.value.clone();
                                info.set(tmp_info);
                                is_confirm.set(false);
                            } }
                        }
    
                        br{}
                        
                        label{
                            "Valor:" br{}
                            input{ r#required:true, r#type: "text", id: "value", oninput: move |price| {
                                let price = price.value.clone();

                                let price = price.replace(",", ".");

                                if !price.is_empty(){
                                    if price.matches(".").count() <= 1{
                                        if let Some(first_char) = price.chars().next() {
                                            if first_char != '.' {
                                                let parse_response = price.trim().parse::<f64>();
                                                
                                                match parse_response{
                                                    Ok(value) => { 
                                                        let mut tmp_info = info.get().clone();

                                                        tmp_info.value = value;
                                                        info.set(tmp_info);
                                                        is_value_valid.set(true);
                                                    }
                                                    Err(_) => is_value_valid.set(false)
                                                }
                                            } else {
                                                is_value_valid.set(false);  
                                            }
                                        } 
                                    }else{
                                        is_value_valid.set(false);
                                    }
                                }

                                is_confirm.set(false);
                            } }
                        }
                        
                        p{ id: "data-invalid", hidden: **is_value_valid, "Valor inválido!" }

                        br{}

                        label{
                            "Data:" br{}
                            input{ r#type: "date", id: "date_in", oninput: move |date_in| {
                                let mut tmp_info = info.get().clone();
                                tmp_info.date_in = date_in.value.trim().parse::<NaiveDate>().unwrap();
                                info.set(tmp_info);
                            } }
                            " - até - "
                            input{ r#type: "date", id: "date_out", oninput: move |date_out| {
                                let mut tmp_info = info.get().clone();
                                tmp_info.date_out = date_out.value.trim().parse::<NaiveDate>().unwrap();
                                info.set(tmp_info);

                                
                                is_confirm.set(false);
                            } }
                        }
    
                        br{}

                        label{
                            "Parcelas:"
                            input{ r#type: "number", id: "installments", r#min: "1", 
                                r#placeholder: "1",
                                oninput: move |entry| {
                                    let installments = entry.value.clone();

                                    if installments.is_empty(){
                                        is_inst_valid.set(false);
                                    }else{
                                        match installments.trim().parse(){
                                            Ok(value) => {
                                                if value == 0{
                                                    is_inst_valid.set(false);
                                                }else{
                                                    let mut tmp_info = info.get().clone();
                                                    tmp_info.installments = value;
                                                    info.set(tmp_info);
        
                                                    is_inst_valid.set(true);
                                                }
                                            },
                                            Err(_) => is_inst_valid.set(false)
                                        }
                                    }

                                    is_confirm.set(false);
                                } 
                            },
                        }
    
                        p{ id: "data-invalid", hidden: **is_inst_valid, "Número de parcelas inválido!" }

                        br{}
    
                        label{
                            "Já tá pago?"
                            input{ r#type: "checkbox", id: "payment", onclick: move |_| {
                                let mut tmp_info = info.get().clone();
                                tmp_info.status = !tmp_info.status;
                                info.set(tmp_info);
                                is_confirm.set(false);
                            } }
                        }
                    }

                    button{ r#type: "submit", id: "confirm-form",
                        onclick: move |_| {
                            let rnt = runtime::Runtime::new().unwrap();

                            if **is_name_valid && **is_value_valid && **is_inst_valid && rnt.block_on(is_complete(&info)){
                                let exists_counts = counts.read().clone();
                                let mut tmp_info = info.get().clone();
                                let mut has_count: bool = true;

                                while has_count{
                                    has_count = false;
                                    let exists_counts = &exists_counts.list;
                                    
                                    for ec in exists_counts{
                                        if tmp_info.id == ec.id{
                                            tmp_info.new_id();
                                            has_count = true;
                                            break;
                                        }
                                    }
                                }
                                let mut tmp_counts = counts.read().clone();

                                tmp_counts.put(tmp_info);

                                counts.write().list = tmp_counts.list.clone();
                                is_confirm.set(true);
                            }
                        },
                        "Confirmar"
                    }

                    p{ hidden: !**is_confirm, "Conta {info.title} adicionada!" }
                }
            }
        
            div{ id: "div-form-buttons",
                hidden: **hidden_paid, 

                h4{ "Pagando" }
                form{
                    p{
                        label{ "Informe o ID da conta a ser paga: "}
                        br{}
                        input{ r#required: true, r#type: "number", r#min: "0",
                            oninput: move |id|{ 
                                is_confirm.set(false);

                                match id.value.trim().parse::<i32>(){
                                    Ok(id) => {
                                        is_id_valid.set(true);
                                        id_search.set(id);
                                    },
                                    Err(_) => is_id_valid.set(false)
                                };
                            }

                        }
                    }

                    p{ id: "data-invalid", hidden: **is_id_valid, "ID inválido!" }

                    button{ id: "confirm-form",
                        r#type: "submit",
                        onclick: move |_| {
                            let mut r = 0;
                            let mut tmp_counts = counts.read().list.clone();

                            for count in tmp_counts.clone(){
                                if count.id == **id_search{
                                    if tmp_counts[r].installments > tmp_counts[r].paid_installments{
                                        tmp_counts[r].paid_installments += 1;
        
                                        println!("{:?}", counts.read().list[r]);
        
                                        if tmp_counts[r].installments == tmp_counts[r].paid_installments{
                                            tmp_counts[r].status = true;   
                                        }
                                    }else {
                                        tmp_counts[r].status = true;
                                    }


                                    has_count = true;
                                    break;
                                }
                                
                                r += 1;
                            }

                            counts.write().list = tmp_counts;

                            if !has_count{
                                is_id_valid.set(false);
                            }else{
                                is_confirm.set(true);
                                is_id_valid.set(true);
                            }

                        },
                        "Confirmar"
                    }

                    p{ hidden: !**is_confirm, "Conta {**id_search} paga!"}
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

                        br{}

                        label{ "Novo valor:"} br{}
                        input{ r#type: "text", placeholder: "Ex.: Frango do Peruzzo" }
                    }

                    button{ id: "confirm-form",
                        onclick: move |_| {
                            is_confirm.set(true);
                        },
                        "Confirmar"
                    }

                    p{ hidden: !**is_confirm, "Conta editada!"}
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
                        
                        br{}

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
                            path.push_str(file.trim());
                            path.push_str(extend.trim());
                            path_export.set(path.clone());

                            let rnt = runtime::Runtime::new().unwrap();
                            rnt.block_on(save_in_file(path.trim(), &counts.read().clone())).unwrap();

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