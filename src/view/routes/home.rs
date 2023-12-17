mod div_options;
mod div_active;
use dioxus_elements::GlobalAttributes;

use crate::structs::Info;
use super::*;

#[derive(Clone, Debug)]
struct Columns{
    name: bool,
    title: bool,
    description: bool,
    date_in: bool,
    date_out: bool,
    paid_installments: bool,
    installments: bool,
    value: bool,
    status: bool
}

impl Columns{
    pub fn new() -> Columns{
        Columns{
            name: true,
            title: true,
            description: false,
            date_in: false,
            date_out: true,
            paid_installments: false,
            installments: false,
            value: true,
            status: true
        }
    }
}

const LINES: usize = 10;

#[component]
pub fn Home (cx: Scope) -> Element {
    let total_counts = use_state(cx, || 0.0);
    let counts: Vec<Info> = Info::test();
    let size_max: usize = counts.len();

    use_shared_state_provider(cx, || Columns::new());
    let columns = use_shared_state::<Columns>(cx).unwrap().read().clone();

    let init = use_state(cx, || 0 as usize);
    let end = use_state(cx, || if size_max > LINES { LINES as usize } else { size_max });
    let page = use_state(cx, || 1);

    
    let mut more: bool = false;
    let mut less: bool = false;
    
    

    render!{

        link{ r#rel: "stylesheet", href: "./src/view/styles/home.css" }

        h2{ id: "name",  "Esmeralda" }

        div{ id: "div-body",
            div_active::div_most(cx),

            // working!!
            // for count in counts{
            //     total_counts.set( total_counts += count.value)
            // }

            div{ id: "div-table",
                table{ id: "table-counts", 
                    th{ format!("Contas: {:2}", total_counts) }
                    tr{ id: "head-table",
                        td{hidden: !columns.name, "Nome" },
                        td{hidden: !columns.title, "Título" },
                        td{hidden: !columns.description, "Descrição" },
                        td{hidden: !columns.date_in, "Data Inicial" },
                        td{hidden: !columns.date_out, "Data Final" },
                        td{hidden: !columns.paid_installments, "Parcelas Pagas" },
                        td{hidden: !columns.installments, "Parcelas" },
                        td{hidden: !columns.value, "Valor" },
                        td{hidden: !columns.status, "Status" }
                    }


                    for i in **init..**end{
                       
                        tr{ 
                            td{hidden: !columns.name, format!("{}", counts[i].debtor) },
                            td{hidden: !columns.title, format!("{}", counts[i].title) },
                            td{hidden: !columns.description, format!("{}", counts[i].description) },
                            td{hidden: !columns.date_in, format!("{}", counts[i].date_in) },
                            td{hidden: !columns.date_out, format!("{}", counts[i].date_out) },
                            td{hidden: !columns.paid_installments, format!("{}", counts[i].paid_installments) },
                            td{hidden: !columns.installments, format!("{}", counts[i].installments) },
                            td{hidden: !columns.value, format!("{:2}", counts[i].value) },
                            td{hidden: !columns.status, id: if counts[i].status { "stt-pos" } else { "stt-neg" } },
                        }
                        
                    }
                }

                if **init == 0{
                    less = true;
                }

                if **end == size_max{
                    more = true;
                }

                div{ id: "move-page",
                    button{ hidden: less, 
                        onclick: move |_| {
                            let (i, e) = back_page(**init, **end);
                            init.set(i);
                            end.set(e);
                            page.set(page - 1);
                        }, 
                        "← Página anterior" 
                    }

                    i{ format!("{page}")}

                    button{ hidden: more, 
                        onclick: move |_| {
                            let (i, e) = next_page(**init, **end, &size_max);
                            init.set(i);
                            end.set(e);
                            page.set(page + 1);
                        }, 
                        "Próxima página →" 
                    }
                }
            }

            div_options::div_options(cx)
        }
    }
}

fn back_page(mut init: usize, mut end: usize) -> (usize, usize) {
    if init > 0{
        end = init;
        init -= LINES;
    }else if init > 0 && init < LINES{
        end = init;
        init += init-0;
    }

    (init, end)
}

fn next_page(mut init: usize, mut end: usize, size_max: &usize) -> (usize, usize) {
    if end <= size_max-LINES{
        init = end;
        end += LINES;
    }else if end < *size_max && end > size_max-LINES{
        init = end;
        end += size_max-end;
    }

    (init, end)
}