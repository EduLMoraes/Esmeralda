use std::fs;
use std::fs::File;
use std::io::Write;
use std::io::BufWriter;
use std::fs::create_dir_all;
use printpdf::*;

use crate::prelude::structs::InterfaceInfo;

#[allow(dead_code)]
async fn mkdir(path: &str) -> (File, String){
    let mut new_path: String = String::new();

    let paths: Vec<&str> = path.split('/').collect();

    for i in 0..paths.len() - 1{
        new_path.push_str(paths[i]);
        new_path.push('/');
    }

    create_dir_all(new_path).unwrap();


    let mut limit: usize = 0;
    for a in path.chars(){
        if a == '.' && limit == 0{
            limit += 1;
        }else if a == '.' {
            break
        }else{
            limit += 1;
        }
    }

    let file: File;
    let mut path = path.to_string();
    let mut is_alterated: bool = false;
    let mut count_files = 0;

    loop{
        if count_files > 0  && count_files < 11{
            if is_alterated{
                path.replace_range(limit..limit+3, format!("({count_files})").trim());
            }else{
                path.insert_str(limit, format!("({count_files})").trim());
                is_alterated = true;
            }
            println!("{path}");
        }else if count_files > 0{
            if is_alterated{
                path.replace_range(limit..limit+4, format!("({count_files})").trim());
            }else{
                path.insert_str(limit, format!("({count_files})").trim());
                is_alterated = true;
            }
            println!("{path}");
        }

        match fs::metadata(path.clone()){
            Ok(_) => count_files += 1,
            Err(_) => {
                file = File::create(path.clone()).unwrap();
                            
                return (file, path)
            }
            
        }
    }


}


#[allow(dead_code)]
pub async fn export_csv( path: &str, data: InterfaceInfo) -> Result<String, String>{
    let data = data.list;

    let (mut file, path) = mkdir(path).await;

    let mut data_file = String::new();

    data_file.push_str("ID,Nome,Titulo,Descricao,Data Inicial,Data Final,Parcelas Pagas,Parcelas,Valor,Status\n");

    for info in data{
        data_file.push_str(format!(
                "{},{},{},{},{},{},{},{},{:.2},{}", 
                info.id,
                info.debtor, 
                info.title,
                info.description,
                info.date_in.to_string(), 
                info.date_out.to_string(),
                info.paid_installments,
                info.installments,
                info.value,
                info.status
            ).trim()
        );
        data_file.push('\n');
    }

    match file.write_all(data_file.as_bytes()) {
        Ok(_) => Ok(path),
        Err(e) => Err(e.to_string()),
    }
}

#[allow(dead_code)]
pub async fn export_html( path: &str, data: InterfaceInfo) -> Result<String, String>{
    let data = data.list;

    let (mut file, path) = mkdir(path).await;

    let mut data_file = String::new();

    data_file.push_str("<html><head></head><body><table>");
    data_file.push_str("<tr>");
    data_file.push_str("<td>ID</td><td>Nome</td><td>Titulo</td><td>Descricao</td><td>Data Inicial</td><td>Data Final</td><td>Parcelas</td><td>Valor</td><td>Status</td>");
    data_file.push_str("</tr>");

    for info in data{
        data_file.push_str("<tr>");

        data_file.push_str(format!(
                "<td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}/{}</td><td>{:.2}</td><td>{}</td>", 
                info.id,
                info.debtor, 
                info.title,
                info.description,
                info.date_in.to_string(), 
                info.date_out.to_string(),
                info.paid_installments,
                info.installments,
                info.value,
                info.status
            ).trim()
        );

        data_file.push_str("</tr>");
    }

    data_file.push_str("</table></body></html>");


    match file.write_all(data_file.as_bytes()) {
        Ok(_) => Ok(path),
        Err(e) => Err(e.to_string()),
    }
}

#[allow(dead_code)]
pub fn export_pdf( path: &str, data: InterfaceInfo) -> Result<String, String>{
    let mut x = Mm(297.0);
    let mut y = Mm(210.0);

    let mut page_count = 1;

    let (doc, page, layer) = PdfDocument::new("PDF_Document_title", x, y, format!("Página {}", page_count));
    let mut current_layer = doc.get_page(page).get_layer(layer);

    let doc = doc.with_title("Emeralda");
    let font = doc.add_external_font(File::open("./assets/fonts/Roboto-Medium.ttf").unwrap()).unwrap();

    y = Mm(205.0);
    let pos_x: Vec<f32> = vec![  5.0, 20.0,   60.0,   100.0,       170.0,         200.0,      230.0,    255.0,  280.0];
    let header: Vec<&str> = vec!["ID","|Nome","|Titulo","|Descricao","|Data Inicial","|Data Final","|Parcelas","|Valor","|Status"];

    for col in 0..9{
        current_layer.use_text(header[col], 12.0, Mm(pos_x[col]), y, &font);
    }

    x = Mm(0.2);
    y -= Mm(2.0);
    for _ in 0..300{
        current_layer.use_text("____", 12.0, x, y, &font);
        x += Mm(2.0);
    }

    for i in 0..data.len(){
        x = Mm(0.2);
        y -= Mm(5.0);
        if y <= Mm(0.0){
            x = Mm(297.0);
            y = Mm(210.0);

            page_count += 1;
            let (page, layer) = doc.add_page(x, y, format!("Página {}", page_count));
            current_layer = doc.get_page(page).get_layer(layer);

            x = Mm(0.2);
            y -= Mm(5.0);
        }

        for col in 0..9{
            let line = vec![
                format!("{}", data.list[i].id),
                format!("| {:.18}", data.list[i].debtor), 
                format!("| {:.30}", data.list[i].title),
                format!("| {:.38}", data.list[i].description),
                format!("| {}", data.list[i].date_in.to_string()), 
                format!("| {}", data.list[i].date_out.to_string()),
                format!("| {}/{}", data.list[i].paid_installments, data.list[i].installments),
                format!("| {:.2}", data.list[i].value),
                format!("| {}", if data.list[i].status {"Pagou"} else {"Deve"})
            ];

            current_layer.use_text(&line[col], 12.0, Mm(pos_x[col]), y, &font);
        }

        for _ in 0..300{
            current_layer.use_text("____", 12.0, x, y, &font);
            x += Mm(2.0);
        }
    }

    doc.save(&mut BufWriter::new(File::create(path).unwrap())).unwrap();

    Ok(String::from(path))
}