use printpdf::*;
use std::fs;
use std::fs::create_dir_all;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

pub use crate::prelude::structs::InterfaceInfo;

/// Creates a directory structure and returns a file and its path. If the directory already exists, it appends a number to the file name to avoid overwriting existing files.
///
/// # Arguments
///
/// * `path` - A string representing the path to the file that needs to be created.
///
/// # Example
///
/// ```rust
/// let (file, path) = mkdir("path/to/file.txt").await;
/// ```
///
/// # Returns
///
/// A tuple containing the created file and its path.
///
/// # Code Analysis
///
/// The function splits the `path` string into individual directory names and creates the directory structure by iterating over the directory names. It then uses the `create_dir_all` function to create all the directories in the `new_path` string.
///
/// The function determines the limit index where the file name starts by counting the characters until the first dot ('.') is encountered. It initializes variables for the file, the altered path, and a flag to track if the path has been altered.
///
/// The function starts a loop to handle cases where the file already exists. If the file count is between 1 and 10, it appends a number to the file name using the `replace_range` or `insert_str` methods. It prints the altered path for debugging purposes.
///
/// The function checks if the file exists using the `fs::metadata` function. If the file does not exist, it creates it using the `File::create` function and returns the file and the path. If the file already exists, it increments the file count and repeats the loop.
///
/// # Errors
///
/// The function may return an error if there are issues with creating directories or files.
///
/// # Safety
///
/// The function is marked as `async` and may need to be used within an asynchronous context.
#[allow(dead_code)]
pub async fn mkdir(path: &str) -> Result<(File, String), String> {
    let mut new_path: String = String::new();

    let paths: Vec<&str> = path.split('/').collect();

    for i in 0..paths.len() - 1 {
        new_path.push_str(paths[i]);
        new_path.push('/');
    }

    match create_dir_all(new_path) {
        Ok(_) => {
            let mut limit: usize = 0;
            for a in path.chars() {
                if a == '.' && limit == 0 {
                    limit += 1;
                } else if a == '.' {
                    break;
                } else {
                    limit += 1;
                }
            }

            let mut path = path.to_string();
            let mut is_alterated: bool = false;
            let mut count_files = 0;

            loop {
                if count_files > 0 && count_files < 11 {
                    if is_alterated {
                        path.replace_range(limit..limit + 3, format!("({count_files})").trim());
                    } else {
                        path.insert_str(limit, format!("({count_files})").trim());
                        is_alterated = true;
                    }
                    println!("{path}");
                } else if count_files > 0 {
                    if is_alterated {
                        path.replace_range(limit..limit + 4, format!("({count_files})").trim());
                    } else {
                        path.insert_str(limit, format!("({count_files})").trim());
                        is_alterated = true;
                    }
                    println!("{path}");
                }

                match fs::metadata(path.clone()) {
                    Ok(_) => count_files += 1,
                    Err(_) => {
                        let file = match File::create(path.clone()) {
                            Ok(f) => f,
                            Err(e) => return Err(e.to_string()),
                        };

                        return Ok((file, path));
                    }
                }
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

/// # export_csv
///
/// A function that takes a `path` and `data` as inputs and returns a `Result` containing either the path of the created file or an error message.
///
/// ## Example Usage
/// ```rust
/// let result = export_csv("path/to/file.csv", &interface_info).await;
/// ```
///
/// ## Arguments
/// - `path` (string): The path to the file that needs to be created.
/// - `data` (InterfaceInfo): The data that will be written to the file.
///
/// ## Returns
/// A `Result` object that contains either the path of the created file or an error message.
///
/// # Code Snippet
#[allow(dead_code)]
pub async fn export_csv(path: &str, data: &InterfaceInfo) -> Result<String, String> {
    let (mut file, path) = mkdir(path).await?;

    let mut data_file = String::new();

    data_file.push_str(
        "ID;Nome;Natureza do gasto;Titulo;Descricao;Data Inicial;Data Final;Parcelas Pagas;Parcelas;Valor;Status\n",
    );

    for info in &data.list {
        data_file.push_str(
            format!(
                "{};{};{};{};{};{};{};{};{};{:.2};{}",
                info.id,
                info.debtor,
                info.nature,
                info.title,
                info.description,
                info.date_in.to_string(),
                info.date_out.to_string(),
                info.paid_installments,
                info.installments,
                info.value,
                info.status
            )
            .trim(),
        );
        data_file.push('\n');
    }

    match file.write_all(data_file.as_bytes()) {
        Ok(_) => Ok(path),
        Err(e) => Err(e.to_string()),
    }
}

/// Export HTML
///
/// The `export_html` function exports data to an HTML file. It takes a `path` and `data` as inputs and returns a `Result` containing either the path of the exported HTML file or an error message.
///
/// # Example Usage
///
/// ```rust
/// let path = "output.html";
/// let data = InterfaceInfo { ... };
///
/// match export_html(path, &data).await {
///     Ok(file_path) => println!("HTML file exported successfully: {}", file_path),
///     Err(error) => println!("Error exporting HTML file: {}", error),
/// }
/// ```
///
/// # Arguments
///
/// * `path` - A string representing the file path where the HTML file will be exported.
/// * `data` - A reference to an `InterfaceInfo` struct containing the data to be included in the HTML table.
///
/// # Returns
///
/// * If the HTML file is exported successfully, the function returns a `Result` containing the path of the exported file.
/// * If there is an error during the file export, the function returns a `Result` containing an error message.
#[allow(dead_code)]
pub async fn export_html(path: &str, data: &InterfaceInfo) -> Result<String, String> {
    let (mut file, path) = mkdir(path).await?;

    let mut data_file = String::new();

    data_file.push_str("<html><head></head><body><table>");
    data_file.push_str("<tr>");
    data_file.push_str("<td>ID</td><td>Nome</td><td>Natureza do gasto</td><td>Titulo</td><td>Descricao</td><td>Data Inicial</td><td>Data Final</td><td>Parcelas</td><td>Valor</td><td>Status</td>");
    data_file.push_str("</tr>");

    for info in &data.list {
        data_file.push_str("<tr>");

        data_file.push_str(
            format!(
                "<td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}/{}</td><td>{:.2}</td><td>{}</td>",
                info.id,
                info.debtor,
                info.nature,
                info.title,
                info.description,
                info.date_in.to_string(),
                info.date_out.to_string(),
                info.paid_installments,
                info.installments,
                info.value,
                info.status
            )
            .trim(),
        );

        data_file.push_str("</tr>");
    }

    data_file.push_str("</table></body></html>");

    match file.write_all(data_file.as_bytes()) {
        Ok(_) => Ok(path),
        Err(e) => Err(e.to_string()),
    }
}

/// Exports data from an `InterfaceInfo` object to a PDF file.
///
/// # Arguments
///
/// * `path` - The path where the PDF file will be saved.
/// * `data` - The data to be exported to the PDF file.
///
/// # Returns
///
/// * `Result<String, String>` - The path of the exported PDF file, or an error message if the export fails.
pub fn export_pdf(path: &str, data: &InterfaceInfo) -> Result<String, String> {
    let mut x = Mm(297.0);
    let mut y = Mm(210.0);

    let mut page_count = 1;

    let (doc, page, layer) = PdfDocument::new("Esmeralda", x, y, format!("Página {}", page_count));
    let mut current_layer = doc.get_page(page).get_layer(layer);

    let font = doc
        .add_external_font(File::open("./assets/fonts/Roboto-Medium.ttf").unwrap())
        .unwrap();

    y = Mm(205.0);
    let pos_x: Vec<f32> = vec![5.0, 20.0, 60.0, 100.0, 170.0, 200.0, 230.0, 255.0, 280.0];
    let header: Vec<&str> = vec![
        "ID",
        "|Nome",
        "|Natureza",
        "|Titulo",
        "|Descricao",
        "|Data Inicial",
        "|Data Final",
        "|Parcelas",
        "|Valor",
        "|Status",
    ];

    for col in 0..9 {
        current_layer.use_text(header[col], 12.0, Mm(pos_x[col]), y, &font);
    }

    x = Mm(0.2);
    y -= Mm(2.0);
    for _ in 0..300 {
        current_layer.use_text("____", 12.0, x, y, &font);
        x += Mm(2.0);
    }

    for i in 0..data.len() {
        x = Mm(0.2);
        y -= Mm(5.0);
        if y <= Mm(0.0) {
            x = Mm(297.0);
            y = Mm(210.0);

            page_count += 1;
            let (page, layer) = doc.add_page(x, y, format!("Página {}", page_count));
            current_layer = doc.get_page(page).get_layer(layer);

            x = Mm(0.2);
            y -= Mm(5.0);
        }

        for col in 0..9 {
            let line = vec![
                format!("{}", data.list[i].id),
                format!("| {:.18}", data.list[i].debtor),
                format!("| {}", data.list[i].nature),
                format!("| {:.30}", data.list[i].title),
                format!("| {:.38}", data.list[i].description),
                format!("| {}", data.list[i].date_in.to_string()),
                format!("| {}", data.list[i].date_out.to_string()),
                format!(
                    "| {}/{}",
                    data.list[i].paid_installments, data.list[i].installments
                ),
                format!("| {:.2}", data.list[i].value),
                format!("| {}", if data.list[i].status { "Pagou" } else { "Deve" }),
            ];

            current_layer.use_text(&line[col], 12.0, Mm(pos_x[col]), y, &font);
        }

        for _ in 0..300 {
            current_layer.use_text("____", 12.0, x, y, &font);
            x += Mm(2.0);
        }
    }

    doc.save(&mut BufWriter::new(File::create(path).unwrap()))
        .unwrap();

    Ok(String::from(path))
}
