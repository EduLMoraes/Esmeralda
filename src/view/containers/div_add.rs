use super::*;
use crate::chrono::NaiveDate;
use crate::control::is_complete;
use crate::control::save;
use crate::alphabetic::is_alphabetic;
use crate::tokio::runtime;

pub fn add(cx: Scope, hidden_add: bool) -> Element{
    let msg = use_shared_state::<Message>(cx).unwrap();
    
    let is_value_valid: &UseState<bool> = use_state(cx, || true);
    let is_inst_valid: &UseState<bool> = use_state(cx, || true);
    let is_name_valid: &UseState<bool> = use_state(cx, || true);

    let counts: &UseSharedState<InterfaceInfo> = use_shared_state::<InterfaceInfo>(cx).unwrap();
    let info: &UseState<Info> = use_state::<Info>(cx, || Info::new());
    let is_new = use_state(cx, || true);
    
    render!(
        div{ id: "div-form-buttons",
            hidden: hidden_add, 

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
                                is_new.set(true);
                                info.set(tmp_info);
                            }   

                            msg.write().hidden = true;

                        }}
                    }
                    
                    p{ id: "data-invalid", hidden: **is_name_valid, "Nome inválido!" }

                    br{}

                    label{
                        "Título:" br{}
                        input{ r#required:true, r#type: "text", id: "title", oninput: move |title| {
                            let mut tmp_info = info.get().clone();
                            tmp_info.title = title.value.clone();
                            is_new.set(true);
                            info.set(tmp_info);

                            msg.write().hidden  = true;

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
                                            let parse_response = price.trim().parse::<f32>();
                                            
                                            match parse_response{
                                                Ok(value) => { 
                                                    let mut tmp_info = info.get().clone();

                                                    tmp_info.value = value;
                                                    is_new.set(true);
                                                    info.set(tmp_info);

                                                    is_value_valid.set( true);
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

                            msg.write().hidden = true;

                        } }
                    }
                    
                    p{ id: "data-invalid", hidden: **is_value_valid, "Valor inválido!" }

                    br{}

                    label{
                        "Data:" br{}
                        input{ r#type: "date", id: "date_in", oninput: move |date_in| {
                            let mut tmp_info = info.get().clone();
                            tmp_info.date_in = date_in.value.trim().parse::<NaiveDate>().unwrap();
                            is_new.set(true);
                            info.set(tmp_info);
                        } } 
                        " - até - "
                        input{ r#type: "date", id: "date_out", oninput: move |date_out| {
                            let mut tmp_info = info.get().clone();
                            tmp_info.date_out = date_out.value.trim().parse::<NaiveDate>().unwrap();
                            is_new.set(true);
                            info.set(tmp_info);

                            
                            msg.write().hidden = true;

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
                                                is_new.set(true);
                                                info.set(tmp_info);
                                                
                                                is_inst_valid.set(true);
                                            }
                                        },
                                        Err(_) => is_inst_valid.set(false)
                                    }
                                }

                                msg.write().hidden = true;

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
                            is_new.set(true);
                            info.set(tmp_info);

                            msg.write().hidden  = true;

                        } }
                    }
                }

                button{ r#type: "submit", id: "confirm-form",
                    onclick: move |_| {
                        let rnt = runtime::Runtime::new().unwrap();

                        if **is_name_valid && **is_value_valid && **is_inst_valid && rnt.block_on(is_complete(&info)) && **is_new{
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
                            info.set(Info::new());

                            is_new.set(false);

                            msg.write().hidden = false;
                            msg.write().text = "Conta adicionada!";

                            let run = tokio::runtime::Runtime::new().unwrap();
                            let response = run.block_on( save( &counts.read() ) );
                            println!("{response:?}");
                        }
                    },
                    "Confirmar"
                }

                p{ hidden: msg.read().hidden, msg.read().text }
            }
        }

    )
}