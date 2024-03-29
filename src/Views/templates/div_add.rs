use super::*;



use crate::prelude::tokio::runtime;
use diacritics::remove_diacritics;

/// Adds a new item to a list by generating a form.
///
/// This function generates a form for adding a new item to a list. The form includes input fields for the name, title, value, date, and number of installments of the item. It also includes a checkbox for indicating whether the item has been paid. The function performs validation on the input fields and updates the state accordingly. When the form is submitted, the function checks if the input is valid and adds the item to the list if it is.
///
/// # Arguments
///
/// * `cx` - A `Scope` object that represents the current scope of the application.
/// * `hidden_add` - A boolean value indicating whether the form should be hidden or not.
///
/// # Example
///
/// ```rust
/// let cx = Scope::new();
/// let hidden_add = false;
/// let element = add(cx, hidden_add);
/// ```
///
/// # Returns
///
/// An `Element` object representing the rendered form.
///
pub fn add(cx: Scope, hidden_add: bool) -> Element {
    let msg = use_shared_state::<Message>(cx).unwrap();
    let cnt = use_shared_state::<Contabilized>(cx).unwrap();
    let path = use_shared_state::<PathBuf>(cx).unwrap();

    let is_value_valid: &UseState<bool> = use_state(cx, || true);
    let is_inst_valid: &UseState<bool> = use_state(cx, || true);
    let is_name_valid: &UseState<bool> = use_state(cx, || true);
    let is_date_valid: &UseState<bool> = use_state(cx, || true);

    let counts: &UseSharedState<InterfaceInfo> = use_shared_state::<InterfaceInfo>(cx).unwrap();
    let info: &UseState<Info> = use_state::<Info>(cx, || Info::new());
    let is_new: &UseState<bool> = use_state(cx, || true);

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

                            is_name_valid.set(control::is_alpha(&name));

                            if **is_name_valid{
                                let mut tmp_info = info.get().clone();
                                tmp_info.debtor = name;
                                is_new.set(true);
                                info.set(tmp_info);
                            }

                            msg.write().hidden = true;

                        }, info.get().debtor.to_string() }
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

                        }, info.get().title.to_string() }
                    }

                    br{}

                    label{
                        "Natureza:" br{}
                        select{
                            onchange: move |column|{
                                msg.write().hidden = true;

                                let column = &column.value.to_lowercase();
                                let column: String = remove_diacritics(column.trim());

                                match column.trim(){
                                    "c" => {
                                        let mut tmp_info = info.get().clone();

                                        tmp_info.nature = String::from("Casa");

                                        is_new.set(true);
                                        info.set(tmp_info);

                                        msg.write().hidden  = true;
                                    },
                                    "t" => {
                                        let mut tmp_info = info.get().clone();

                                        tmp_info.nature = String::from("Transporte");

                                        is_new.set(true);
                                        info.set(tmp_info);

                                        msg.write().hidden  = true;
                                    },
                                    "i" => {
                                        let mut tmp_info = info.get().clone();

                                        tmp_info.nature = String::from("Investimentos");

                                        is_new.set(true);
                                        info.set(tmp_info);

                                        msg.write().hidden  = true;
                                    },
                                    "s" => {
                                        let mut tmp_info = info.get().clone();

                                        tmp_info.nature = String::from("Saúde");

                                        is_new.set(true);
                                        info.set(tmp_info);

                                        msg.write().hidden  = true;
                                    },
                                    "l" => {
                                        let mut tmp_info = info.get().clone();

                                        tmp_info.nature = String::from("Lazer");

                                        is_new.set(true);
                                        info.set(tmp_info);

                                        msg.write().hidden  = true;
                                    },
                                    "a" => {
                                        let mut tmp_info = info.get().clone();

                                        tmp_info.nature = String::from("Alimentação");

                                        is_new.set(true);
                                        info.set(tmp_info);

                                        msg.write().hidden  = true;
                                    },
                                    "o" => {
                                        let mut tmp_info = info.get().clone();

                                        tmp_info.nature = String::from("Outros");

                                        is_new.set(true);
                                        info.set(tmp_info);

                                        msg.write().hidden  = true;
                                    },
                                    _ => {}
                                }
                            },

                            option{ value: "a", "Alimentação"}
                            option{ value: "c", "Casa"}
                            option{ value: "i", "Investimentos"}
                            option{ value: "l", "Lazer"}
                            option{ value: "t", "Transporte"}
                            option{ value: "s", "Saúde"}
                            option{ value: "o", "Outros" }
                        }
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

                        }, info.get().value.to_string() }
                    }

                    p{ id: "data-invalid", hidden: **is_value_valid, "Valor inválido!" }

                    br{}

                    label{
                        "Data:" br{}
                        input{ r#type: "date", id: "date_in",
                            r#placeholder: "{info.get().date_in.to_string()}",
                            oninput: move |date_in| {
                                let mut tmp_info = info.get().clone();
                                let input_data = date_in.value.trim().parse::<NaiveDate>().unwrap();

                                if input_data <= tmp_info.date_out{
                                    is_date_valid.set(true);

                                    tmp_info.date_in = input_data;
                                    let installments = control::signed_month_difference(tmp_info.date_in, tmp_info.date_out) + 1;
                                    tmp_info.installments = installments as u32;

                                    is_new.set(true);
                                    info.set(tmp_info);
                                } else{
                                    is_date_valid.set(false);
                                }
                        }, info.get().date_in.to_string() }
                        " - até - "
                        input{ r#type: "date", id: "date_out",
                            r#placeholder: "{info.get().date_out.to_string()}",
                            oninput: move |date_out| {
                                let mut tmp_info = info.get().clone();
                                let input_data = date_out.value.trim().parse::<NaiveDate>().unwrap();

                                if input_data >= tmp_info.date_in{
                                    is_date_valid.set(true);

                                    tmp_info.date_out = input_data;
                                    let installments = control::signed_month_difference(tmp_info.date_in, tmp_info.date_out) + 1;
                                    tmp_info.installments = installments as u32;

                                    is_new.set(true);
                                    info.set(tmp_info);
                                    msg.write().hidden = true;

                                }else{
                                    is_date_valid.set(false);
                                }
                        }, info.get().date_out.to_string()}
                    }
                    p{ id: "data-invalid", hidden: **is_date_valid, "Datas inválidas!"}

                    br{}

                    label{
                        "Parcelas:"
                        input{ r#type: "number", id: "installments", r#min: "1",
                            r#placeholder: "{info.get().installments.to_string()}",
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

                                                let dif_time = control::signed_month_difference(tmp_info.date_in, tmp_info.date_out) + 1;

                                                if dif_time < tmp_info.installments as i32{
                                                    tmp_info.date_out = tmp_info.date_out.checked_add_months(Months::new(1)).unwrap();
                                                }else if dif_time > tmp_info.installments as i32{
                                                    tmp_info.date_out = tmp_info.date_out.checked_sub_months(Months::new(1)).unwrap();
                                                }

                                                is_new.set(true);
                                                info.set(tmp_info);

                                                is_inst_valid.set(true);
                                            }
                                        },
                                        Err(_) => is_inst_valid.set(false)
                                    }
                                }

                                msg.write().hidden = true;

                            },
                            info.get().installments.to_string()
                        },
                    }

                    p{ id: "data-invalid", hidden: **is_inst_valid, "Número de parcelas inválido!" }

                    br{}

                    label{
                        "Já tá pago?"
                        input{ r#type: "checkbox", id: "payment", onclick: move |_| {
                            let mut tmp_info = info.get().clone();
                            tmp_info.status = !tmp_info.status;

                            if tmp_info.status{
                                tmp_info.paid_installments = tmp_info.installments;
                            }else{
                                tmp_info.paid_installments = 0;
                            }

                            is_new.set(true);
                            info.set(tmp_info);

                            msg.write().hidden  = true;

                        }, value: info.get().status }
                    }
                }

                button{ r#type: "reset", id: "confirm-form",
                    onclick: move |_| {
                        let rnt = runtime::Runtime::new().unwrap();

                        if **is_name_valid && **is_value_valid && **is_inst_valid && **is_date_valid && rnt.block_on(control::is_complete(&info)){
                            let exists_counts = counts.read().clone();
                            let mut tmp_info = info.get().clone();
                            let mut has_count: bool = true;

                            if tmp_info.nature.is_empty(){
                                tmp_info.nature = String::from("Alimentação");
                            }

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
                            let response = run.block_on( control::save( &counts.read() ) );

                            *cnt.write() = Contabilized::No;

                            if response.is_err(){
                                let _ = log(path.read().clone(), &format!("[CONTAINER ADD] {response:?}\n"));
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
