use super::*;

use crate::prelude::control::save;

/// Handles the payment of an account.
///
/// # Arguments
///
/// * `cx` - A scope object that represents the current scope.
/// * `hidden_paid` - A boolean indicating whether the payment is hidden.
///
/// # Returns
///
/// An element representing the rendered HTML.
///
/// # Example
///
/// ```rust
/// let scope = Scope::new();
/// let hidden_paid = false;
/// let element = paid(scope, hidden_paid);
/// ```
pub fn paid(cx: Scope, hidden_paid: bool) -> Element {
    let msg = use_shared_state::<Message>(cx).unwrap();
    let cnt = use_shared_state::<Contabilized>(cx).unwrap();
    let path = use_shared_state::<PathBuf>(cx).unwrap();

    let is_id_valid: &UseState<bool> = use_state(cx, || true);
    let id_search: &UseState<i32> = use_state(cx, || 0);

    let counts: &UseSharedState<InterfaceInfo> = use_shared_state::<InterfaceInfo>(cx).unwrap();

    render!(
        div{ id: "div-form-buttons",
            hidden: hidden_paid,

            h4{ "Pagando" }
            form{
                onsubmit: move |_|{
                    let run = tokio::runtime::Runtime::new().unwrap();
                    let response = run.block_on( save( &counts.read() ) );
                    match response{
                        Ok(_) => {},
                        Err(err) => { let _ = log(path.read().clone(), &format!("[CONTAINER PAID] {err}\n")); }
                    }
                },
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
                        let mut has_count: bool = false;

                        for count in tmp_counts.clone(){
                            if count.id == **id_search{
                                if tmp_counts[r].installments > tmp_counts[r].paid_installments{
                                    tmp_counts[r].paid_installments += 1;


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

                            let run = tokio::runtime::Runtime::new().unwrap();
                            let response = run.block_on( control::edit( &counts.read().clone() ) );
                            *cnt.write() = Contabilized::No;

                            if response.is_err(){
                                let _ = log(path.read().clone(), &format!("[CONTAINER PAID] {response:?}\n"));
                            }
                        }

                    },
                    "Confirmar"
                }

                p{ hidden: msg.read().hidden, msg.read().text }
            }
        }

    )
}
