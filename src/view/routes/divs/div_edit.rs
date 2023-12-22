use super::*;
use diacritics::remove_diacritics;
use crate::chrono::NaiveDate;
use crate::prelude::controller::is_alphabetic;

pub fn edit(cx: Scope, hidden_edit: bool) -> Element{
    let msg = use_shared_state::<Message>(cx).unwrap();

    let is_id_valid: &UseState<bool> = use_state(cx, || true);
    let is_name_valid: &UseState<bool> = use_state(cx, || true);

    let is_value: &UseState<bool> = use_state(cx, || false);
    let is_installment: &UseState<bool> = use_state(cx, || false);
    let is_date: &UseState<bool> = use_state(cx, || false);
    let is_column: &UseState<bool> = use_state(cx, || true);
    let is_compatible: &UseState<bool> = use_state(cx, || true);
    let is_confirm: &UseState<bool> = use_state(cx, || false);

    let has_id = use_state(cx, || false);
    let has_column = use_state(cx, || false);
    let has_new_value = use_state(cx, || false);

    let type_input: &UseState<&str> = use_state(cx, || "text");

    let id_search: &UseState<i32> =  use_state(cx, || 0);
    let col_search: &UseState<String> = use_state(cx, || String::new());
    let new_value: &UseState<String> = use_state(cx, || String::new());

    render!(
        div{ id: "div-form-buttons",
            hidden: hidden_edit, 

            h4{ "Editando conta "}
            form{
                p{
                    label{ "ID:" } br{}
                    input{ r#type: "number", r#min: "0", r#required: true, oninput: move |id| {
                        is_confirm.set(false);
                        has_id.set(false);

                        match id.value.trim().parse::<i32>(){
                            Ok(id) => {
                                is_id_valid.set(true);
                                id_search.set(id);
                                has_id.set(true);
                            },
                            Err(_) => is_id_valid.set(false)
                        };
                    } }

                    p{ id: "data-invalid", hidden: **is_id_valid, "ID inválido!" }


                    br{}

                    label{ "Coluna:" } br{}
                    input{ r#type: "text", placeholder: "Ex.: Título", r#required: true, oninput: move |col| {
                        has_column.set(false);
                        is_confirm.set(false);
                        let column = col.value.to_lowercase();
                        let column: String = remove_diacritics(column.trim());

                        match column.trim(){
                            "nome" => {
                                has_column.set(true);
                                is_column.set(true);
                                type_input.set("text");
                                col_search.set( String::from("debtor") )
                            },
                            "titulo" => {
                                has_column.set(true);
                                is_column.set(true);
                                type_input.set("text");
                                col_search.set( String::from("title") )
                            },
                            "descricrao" => {
                                has_column.set(true);
                                is_column.set(true);
                                type_input.set("text");
                                col_search.set( String::from("description") )
                            },
                            "data inicial" => {
                                has_column.set(true);
                                is_column.set(true);
                                type_input.set("date");
                                col_search.set( String::from("date_in") )
                            },
                            "data final" => {
                                has_column.set(true);
                                is_column.set(true);
                                type_input.set("date");
                                col_search.set( String::from("date_out") )
                            },
                            "parcelas pagas" => {
                                has_column.set(true);
                                is_column.set(true);
                                type_input.set("number");
                                col_search.set( String::from("paid_installments") )
                            },
                            "parcelas" => {
                                has_column.set(true);
                                is_column.set(true);
                                type_input.set("number");
                                col_search.set( String::from("installments") )
                            },
                            "valor" => {
                                has_column.set(true);
                                is_column.set(true);
                                type_input.set("number");
                                col_search.set( String::from("value") )
                            },
                            _ => is_column.set(false)
                        }
                    } }
                    
                    p{ id: "data-invalid", hidden: **is_column, "Coluna inválida!" }

                    br{}

                    label{ "Novo valor:"} br{}
                    input{ r#type: *type_input.get(), placeholder: "Ex.: Frango do Peruzzo", r#required: true, oninput: move |inp| {
                        let value = &inp.value;
                        let column = col_search.get();

                        is_installment.set(false);
                        is_value.set(false);
                        is_date.set(false);
                        is_confirm.set(false);
                        has_new_value.set(false);

                        if column == "debtor"{
                            is_name_valid.set(is_alphabetic(value));

                            if **is_name_valid{
                                is_compatible.set(true);
                                has_new_value.set(true);
                                
                                new_value.set( value.to_string() )
                            }else{
                                is_compatible.set(false)
                            }
                        }else if column == "date_out" || column == "date_in"{

                            match value.trim().parse::<NaiveDate>(){
                                Ok(date) =>{
                                    is_compatible.set(true);
                                    has_new_value.set(true);
                                    is_date.set(true);
                                    new_value.set( date.to_string() )
                                },
                                Err(_) => is_compatible.set(false)
                            }

                        }else if column == "value"{
                            let price = value.replace(",", ".");

                            if !price.is_empty(){
                                if price.matches(".").count() <= 1{
                                    if let Some(first_char) = price.chars().next() {
                                        if first_char != '.' {
                                            let parse_response = price.trim().parse::<f64>();
                                            
                                            match parse_response{
                                                Ok(value) => { 
                                                    is_value.set(true);
                                                    new_value.set( value.to_string() );

                                                    is_compatible.set(true);
                                                    has_new_value.set(true);
                                                }
                                                Err(_) => is_compatible.set(false)
                                            }
                                        } else {
                                            is_compatible.set(false);  
                                        }
                                    } 
                                }else{
                                    is_compatible.set(false);
                                }
                            }
                        }else if column == "paid_installments" || column == "installments"{
                            if value.is_empty(){
                                is_compatible.set(false);
                            }else{
                                match value.trim().parse::<u32>(){
                                    Ok(value) => {
                                        if value == 0{
                                            is_compatible.set(false);
                                        }else{
                                            is_installment.set( true );
                                            new_value.set( value.to_string() );

                                            is_compatible.set(true);
                                            has_new_value.set(true);
                                        }
                                    },
                                    Err(_) => is_compatible.set(false)
                                }
                            }

                        }
                    } }

                    p{ id: "data-invalid", hidden: **is_compatible, "Valor inválido!" }


                }

                button{ id: "confirm-form", r#type: "submit",
                    onclick: move |_| {
                        if **has_id && **has_column && **has_new_value{
                            is_confirm.set(true);

                            has_id.set(false);
                            has_column.set(false);
                            has_new_value.set(false);

                            msg.write().hidden = false;
                            msg.write().text = "Conta editada!";
                        }
                    },
                    "Confirmar"
                }

                p{ hidden: msg.read().hidden, msg.read().text }
            }
        }

    )
}