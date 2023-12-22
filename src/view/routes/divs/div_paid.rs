use super::*;

pub fn paid(cx: Scope, hidden_paid: bool) -> Element{
    let msg = use_shared_state::<Message>(cx).unwrap();

    let is_id_valid: &UseState<bool> = use_state(cx, || true);
    let id_search: &UseState<i32> =  use_state(cx, || 0);
    let mut has_count: bool = false;

    let counts: &UseSharedState<InterfaceInfo> = use_shared_state::<InterfaceInfo>(cx).unwrap();

    render!(
        div{ id: "div-form-buttons",
            hidden: hidden_paid, 

            h4{ "Pagando" }
            form{
                p{
                    label{ "Informe o ID da conta a ser paga: "}
                    br{}
                    input{ r#required: true, r#type: "number", r#min: "0",
                        oninput: move |id|{ 
                            msg.write().hidden = true;

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
                            msg.write().hidden = false;
                            msg.write().text = "Conta paga!";
                            is_id_valid.set(true);
                        }

                    },
                    "Confirmar"
                }

                p{ hidden: msg.read().hidden, msg.read().text }
            }
        }
   
    )
}