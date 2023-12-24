use super::*;
use diacritics::remove_diacritics;
use crate::chrono::NaiveDate;
use crate::alphabetic::is_alphabetic;
use crate::control;
use crate::tokio;

/// Renders an HTML form for editing a specific account.
///
/// # Arguments
///
/// * `cx` - The scope object used for managing the state of the form.
/// * `hidden_edit` - A boolean value indicating whether the form should be hidden or not.
///
/// # Returns
///
/// The rendered HTML element representing the edit form.
///
/// # Example
///
/// ```rust
/// let cx = Scope::new();
/// let hidden_edit = false;
/// let element = edit(cx, hidden_edit);
/// ```
///
/// # Code Analysis
///
/// This function initializes various state variables using the `use_state` and `use_shared_state` functions.
/// It renders an HTML form using the `render!` macro, which includes input fields for the account ID, column selection, and new value.
/// The function also includes event handlers for input changes and form submission.
/// The event handlers update the state variables based on the user's input and perform validation checks.
/// If all the inputs are valid, the function updates the corresponding account information and displays a success message.
/// The function also calls an external function `control::edit` to perform the actual account editing.
///
pub fn edit(cx: Scope, hidden_edit: bool) -> Element{
    let msg = use_shared_state::<Message>(cx).unwrap();
    let counts: &UseSharedState<InterfaceInfo> = use_shared_state::<InterfaceInfo>(cx).unwrap();

    let is_id_valid: &UseState<bool> = use_state(cx, || true);
    let is_name_valid: &UseState<bool> = use_state(cx, || true);

    let is_value: &UseState<bool> = use_state(cx, || false);
    let is_installment: &UseState<bool> = use_state(cx, || false);
    let is_date: &UseState<bool> = use_state(cx, || false);
    let is_column: &UseState<bool> = use_state(cx, || true);
    let is_compatible: &UseState<bool> = use_state(cx, || true);

    let has_id: &UseState<bool> = use_state(cx, || false);
    let has_column: &UseState<bool> = use_state(cx, || true);
    let has_new_value: &UseState<bool> = use_state(cx, || false);

    let type_input: &UseState<&str> = use_state(cx, || "text");

    let id_search: &UseState<i32> =  use_state(cx, || 0);
    let col_search: &UseState<String> = use_state(cx, || "debtor".to_string());
    let new_value: &UseState<String> = use_state(cx, || String::new());



    render!(
        div{ id: "div-form-buttons",
            hidden: hidden_edit, 

            h4{ "Editando conta "}
            form{
                p{
                    label{ "ID:" } br{}
                    input{ r#type: "number", r#min: "0", r#required: true, oninput: move |id| {
                        msg.write().hidden = true;
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
                    select{
                        onchange: move |column|{
                            has_column.set(false);
                            msg.write().hidden = true;
                            has_new_value.set(false);

                            let column = &column.value.to_lowercase();
                            let column: String = remove_diacritics(column.trim());

                            println!("{column}");

                            match column.trim(){
                                "n" => {
                                    has_column.set(true);
                                    is_column.set(true);
                                    type_input.set("text");
                                    col_search.set( String::from("debtor") )
                                },
                                "t" => {
                                    has_column.set(true);
                                    is_column.set(true);
                                    type_input.set("text");
                                    col_search.set( String::from("title") )
                                },
                                "d" => {
                                    has_column.set(true);
                                    is_column.set(true);
                                    type_input.set("text");
                                    col_search.set( String::from("description") )
                                },
                                "di" => {
                                    has_column.set(true);
                                    is_column.set(true);
                                    type_input.set("date");
                                    col_search.set( String::from("date_in") )
                                },
                                "df" => {
                                    has_column.set(true);
                                    is_column.set(true);
                                    type_input.set("date");
                                    col_search.set( String::from("date_out") )
                                },
                                "pp" => {
                                    has_column.set(true);
                                    is_column.set(true);
                                    type_input.set("number");
                                    col_search.set( String::from("paid_installments") )
                                },
                                "p" => {
                                    has_column.set(true);
                                    is_column.set(true);
                                    type_input.set("number");
                                    col_search.set( String::from("installments") )
                                },
                                "v" => {
                                    has_column.set(true);
                                    is_column.set(true);
                                    type_input.set("number");
                                    col_search.set( String::from("value") )
                                },
                                _ => is_column.set(false)
                            }
                        },

                        option{ value: "n", "Nome"}
                        option{ value: "t", "Título"}
                        option{ value: "d", "Descrição"}
                        option{ value: "di", "Data Inicial"}
                        option{ value: "df", "Data Final"}
                        option{ value: "pp", "Parcelas Pagas"}
                        option{ value: "p", "Parcelas" }
                        option{ value: "v", "Valor" }
                    }
                    
                    p{ id: "data-invalid", hidden: **is_column, "Coluna inválida!" }

                    br{}

                    label{ "Novo valor:"} br{}
                    input{ r#type: *type_input.get(), placeholder: "Ex.: Frango do Peruzzo", r#required: true, oninput: move |inp| {
                        let value = &inp.value;
                        let column = col_search.get();

                        is_installment.set(false);
                        is_value.set(false);
                        is_date.set(false);
                        msg.write().hidden = true;
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
                                            let parse_response = price.trim().parse::<f32>();
                                            
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
                        }else if column == "title" || column == "description"{
                            is_compatible.set(true);
                            has_new_value.set(true);
                            
                            new_value.set( value.to_string() )
                        }
                    }, new_value.get().clone() }

                    p{ id: "data-invalid", hidden: **is_compatible, "Valor inválido!" }
                }

                button{ id: "confirm-form", r#type: "submit",
                    onclick: move |_| {
                        if **has_id && **has_column && **has_new_value{

                            let mut r = 0;
                            let mut has_count: bool = false;
                            let tmp_counts = counts.read().list.clone();

                            for count in tmp_counts{
                                if count.id == **id_search{
                                    match col_search.get().trim(){
                                        "debtor"            => { counts.write().list[r].debtor = new_value.get().clone() },
                                        "title"             => { counts.write().list[r].title = new_value.get().clone() },
                                        "description"       => { counts.write().list[r].description = new_value.get().clone() },
                                        "date_in"           => { counts.write().list[r].date_in = new_value.get().clone().parse::<NaiveDate>().unwrap() },
                                        "date_out"          => { counts.write().list[r].date_out = new_value.get().clone().parse::<NaiveDate>().unwrap() },
                                        "value"             => { counts.write().list[r].value = new_value.get().clone().parse::<f32>().unwrap() },
                                        "paid_installments" => { counts.write().list[r].paid_installments = new_value.get().clone().parse::<u32>().unwrap() },
                                        "installments"      => { counts.write().list[r].installments = new_value.get().clone().parse::<u32>().unwrap()} ,
                                        _ => println!("Coluna inválida")
                                    }

                                    has_count = true;
                                    break;
                                }
                                
                                r += 1;
                            }

                            if !has_count{
                                is_id_valid.set(false);
                            }else{
                                msg.write().hidden = false;
                                msg.write().text = "Conta editada!";
                                is_id_valid.set(true);

                                let run = tokio::runtime::Runtime::new().unwrap();
                                let response = run.block_on( control::edit( &counts.read().clone() ) );
                                println!("{:?}", response);
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