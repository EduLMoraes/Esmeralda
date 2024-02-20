use templates::{ div_active, div_options, Columns, Contabilized};
use super::*;
use crate::prelude::control;
use crate::prelude::log;
use crate::prelude::model::list::InterfaceInfo;
use crate::prelude::tokio;
use crate::prelude::Instant;
use std::path::PathBuf;
use styles::home::*;

/// Renders a web page for the 'Home' component.
///
/// # Arguments
///
/// * `cx` - The scope object that provides access to the component's context.
///
/// # Returns
///
/// An `Element` representing the rendered web page.
///
/// # Example
///
/// ```rust
/// #[component]
/// pub fn Home(cx: Scope) -> Element {
///     // function implementation
/// }
/// ```
///
/// # Code Analysis
///
/// This function renders a web page using various state and shared state variables to manage the data and UI elements on the page. It follows the following steps:
///
/// 1. Creates a new Tokio runtime and uses it to execute a blocking operation to recover the shared state.
/// 2. Retrieves the shared state of `InterfaceInfo` and clones its value.
/// 3. Calculates the maximum size of the data and initializes a state variable `contabilized` with this value.
/// 4. Initializes state variables for `total_debt`, `total_paid`, `total_debt_st`, `total_paid_st`, and `total_counts`.
/// 5. Sets the shared state of `Columns` to a new instance of `Columns`.
/// 6. Retrieves the value of `Columns` shared state and clones it.
/// 7. Initializes state variables for `init`, `end`, and `page`.
/// 8. If the `total_counts` is 0 and the size is greater than 0 or the size has changed, it calculates the total debt and total paid amounts and updates the state variables accordingly.
/// 9. Initializes state variables for `crescent`, `more`, and `less`.
/// 10. Renders the HTML elements using the `render!` macro.
///
#[component]
pub fn Home(cx: Scope) -> Element {
    let run = tokio::runtime::Runtime::new().unwrap();
    let path = use_shared_state::<PathBuf>(cx).unwrap();

    use_shared_state_provider(cx, || {
        let now = Instant::now();
        let recovered = run.block_on(control::recover()).unwrap();
        let _ = log(
            path.read().clone(),
            &format!(
                "[HOME] Recover InterfaceInfo in...[{:.3?}]\n",
                now.elapsed()
            ),
        );

        recovered.order_by_status(true)
    });

    use_shared_state_provider(cx, || Contabilized::No);

    let counts = use_shared_state::<InterfaceInfo>(cx).unwrap();
    let counts_info = counts.read().clone();

    let contabilized = use_shared_state::<Contabilized>(cx).unwrap();

    let mut total_debt: f32 = 0.0;
    let mut total_paid: f32 = 0.0;
    let total_debt_st: &UseState<f32> = use_state(cx, || 0.0);
    let total_paid_st: &UseState<f32> = use_state(cx, || 0.0);
    let total_counts: &UseState<f32> = use_state(cx, || 0.0);

    use_shared_state_provider(cx, || Columns::new());
    let columns: Columns = use_shared_state::<Columns>(cx).unwrap().read().clone();

    let list_counts = counts.read().order_alphabetical("debtor", true);
    let debtors = list_counts.filter_debtors();

    let now = Instant::now();

    if *contabilized.read() == Contabilized::No {
        *contabilized.write() = Contabilized::Yes;

        for debtor in debtors {
            total_paid += debtor.get_value();
            total_debt += debtor.get_debt();
        }

        total_debt_st.set(total_debt);
        total_paid_st.set(total_paid);
        total_counts.set(total_debt + total_paid);
    }

    let _ = log(
        path.read().clone(),
        &format!("[HOME] Counts contabilized in...[{:.3?}]\n", now.elapsed()),
    );

    let crescent: &UseState<bool> = use_state(cx, || false);

    let test_search = use_state(cx, || String::new());

    render! {

        style {{ style_home() }}

        h2{ id: "name",  "Esmeralda" }

        div{ id: "div-body",
            div_active::div_most(cx),

            div_options::div_options(cx),
            
            div{ id: "div-right",
                
                input{
                    id: "search-bar",
                    r#type: "text",
                    oninput: move |item|{
                        test_search.set( item.value.clone() )
                    }
                }
                br{}
                format!("Contas: total: R${:.2} | a pagar: R${:.2} | pago: R${:.2}", **total_counts, **total_debt_st, **total_paid_st)
                
                div{ id: "div-table",
                    table{ id: "table-counts",
                        tr{ id: "head-table",
                            td{ id: "col-id",
                                button{
                                    id: "button-order",
                                    onclick: move |_| {
                                        let ci =  counts.read().clone();
    
                                        let now = Instant::now();
                                        counts.write().list = ci.order_by_id(**crescent).list;
                                        crescent.set(!**crescent);
    
                                        let _ = log(path.read().clone(), &format!("[HOME] Ordened table by id in...[{:.3?}]\n", now.elapsed()));
    
                                    }, "ID"
                                }
                            },
                            td{ id: "with-button",
                                hidden: !columns.name,
                                button{
                                    id: "button-order",
                                    hidden: !columns.name, onclick: move |_| {
                                        let ci =  counts.read().clone();
    
                                        let now = Instant::now();
                                        counts.write().list = ci.order_alphabetical("name", **crescent).list;
                                        crescent.set(!**crescent);
    
                                        let _ = log(path.read().clone(), &format!("[HOME] Ordened table by name in...[{:.3?}]\n", now.elapsed()));
                                    }, "Nome"
                                }
                            },
                            td{ id: "with-button",
                                hidden: !columns.nature,
                                button{
                                    id: "button-order",
                                    hidden: !columns.nature, onclick: move |_| {
                                        let ci =  counts.read().clone();
    
                                        let now = Instant::now();
                                        counts.write().list = ci.order_alphabetical("nature", **crescent).list;
                                        crescent.set(!**crescent);
    
                                        let _ = log(path.read().clone(), &format!("[HOME] Ordened table by nature in...[{:.3?}]\n", now.elapsed()));
    
                                    }, "Natureza do gasto"
                                }
                            },
                            td{ id: "with-button",
                                hidden: !columns.title,
                                button{
                                    id: "button-order",
                                    hidden: !columns.title,
                                    onclick: move |_|{
                                        let ci =  counts.read().clone();
    
                                        let now = Instant::now();
                                        counts.write().list = ci.order_alphabetical("title", **crescent).list;
                                        crescent.set(!**crescent);
    
                                        let _ = log(path.read().clone(), &format!("[HOME] Ordened table by title in...[{:.3?}]\n", now.elapsed()));
    
                                    }, "Título"
                                }
                            },
                            td{ id: "with-button",
                                hidden: !columns.description,
                                button{
                                    id: "button-order",
                                    hidden: !columns.description, onclick: move |_| {
                                        let ci =  counts.read().clone();
    
                                        let now = Instant::now();
                                        counts.write().list = ci.order_alphabetical("description", **crescent).list;
                                        crescent.set(!**crescent);
    
                                        let _ = log(path.read().clone(), &format!("[HOME] Ordened table by description in...[{:.3?}]\n", now.elapsed()));
    
                                    }, "Descrição"
                                }
                            },
                            td{ id: "with-button",
                                hidden: !columns.date_in,
                                button{
                                    id: "button-order",
                                    hidden: !columns.date_in, onclick: move |_| {
                                        let ci =  counts.read().clone();
    
                                        let now = Instant::now();
                                        counts.write().list = ci.order_by_date(true, **crescent).list;
                                        crescent.set(!**crescent);
    
                                        let _ = log(path.read().clone(), &format!("[HOME] Ordened table by date_in in...[{:.3?}]\n", now.elapsed()));
    
                                    }, "Data Inicial"
                                }
                            },
                            td{ id: "with-button",
                                hidden: !columns.date_out,
                                button{
                                    id: "button-order",
                                    hidden: !columns.date_out, onclick: move |_| {
                                        let ci =  counts.read().clone();
    
                                        let now = Instant::now();
                                        counts.write().list = ci.order_by_date(false, **crescent).list;
                                        crescent.set(!**crescent);
    
                                        let _ = log(path.read().clone(), &format!("[HOME] Ordened table by date_out in...[{:.3?}]\n", now.elapsed()));
    
                                    }, "Data Final"
                                }
                            },
                            td{ id: "with-button",
                                hidden: !columns.installments,
                                button{
                                    id: "button-order",
                                    hidden: !columns.installments, onclick: move |_| {
                                        let ci =  counts.read().clone();
    
                                        let now = Instant::now();
                                        counts.write().list = ci.order_by_installments(false, **crescent).list;
                                        crescent.set(!**crescent);
    
                                        let _ = log(path.read().clone(), &format!("[HOME] Ordened table by installments in...[{:.3?}]\n", now.elapsed()));
    
                                    }, "Parcelas Pago/Total"
                                }
                            },
                            td{ id: "with-button",
                                hidden: !columns.value,
                                button{
                                    id: "button-order",
                                    hidden: !columns.value, onclick: move |_| {
                                        let ci =  counts.read().clone();
    
                                        let now = Instant::now();
                                        counts.write().list = ci.order_by_value(**crescent).list;
                                        crescent.set(!**crescent);
    
                                        let _ = log(path.read().clone(), &format!("[HOME] Ordened table by value in...[{:.3?}]\n", now.elapsed()));
    
                                    }, "Valor p/ parcela"
                                }
                            },
                            td{ id: "with-button",
                                hidden: !columns.status,
                                button{
                                    id: "button-order",
                                    hidden: !columns.status, onclick: move |_| {
                                        let ci =  counts.read().clone();
    
                                        let now = Instant::now();
                                        counts.write().list = ci.order_by_status(**crescent).list;
                                        crescent.set(!**crescent);
    
                                        let _ = log(path.read().clone(), &format!("[HOME] Ordened table by status in...[{:.3?}]\n", now.elapsed()));
                                    }, "Status"
                                }
                            }
                        }
    
                      
                        for info in counts_info.search(test_search.to_string()){
    
                            tr{
                                td{ id: "col-id", format!("{}", info.id) },
                                td{ id: "col-name", hidden: !columns.name, format!("{}", info.debtor) },
                                td{ id: "col-nature", hidden: !columns.nature, format!("{}", info.nature) },
                                td{ id: "col-title", hidden: !columns.title, format!("{}", info.title) },
                                td{ id: "col-description", hidden: !columns.description, format!("{}", info.description) },
                                td{ id: "col-date", hidden: !columns.date_in, format!("{}", info.date_in) },
                                td{ id: "col-date", hidden: !columns.date_out, format!("{}", info.date_out) },
                                td{ id: "col-inst", hidden: !columns.installments, format!("{}/{}", info.paid_installments, info.installments) },
                                td{ id: "col-value", hidden: !columns.value, format!("{:.2}", info.value) },
                                td{ hidden: !columns.status, div{ id: if info.status { "stt-pos" } else { "stt-neg" } } },
                            }
    
                        }
    
                    }
    
                }
                
            }

        }
    }
}
