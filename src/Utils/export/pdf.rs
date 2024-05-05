use super::*;

/// Exports data from an `ListCount` object to a PDF file.
///
/// # Arguments
///
/// * `path` - The path where the PDF file will be saved.
/// * `data` - The data to be exported to the PDF file.
///
/// # Returns
///
/// * `Result<String, String>` - The path of the exported PDF file, or an error message if the export fails.
#[allow(deprecated, unused)]
pub fn export_pdf(path: &str, data: &ListCount) -> Result<String, String> {
    let mut x = Mm(297.0);
    let mut y = Mm(210.0);

    let mut page_count = 1;

    let (doc, page, layer) = PdfDocument::new("Esmeralda", x, y, format!("Página {}", page_count));
    let mut current_layer = doc.get_page(page).get_layer(layer);

    let font = doc
        .add_external_font(File::open("./assets/fonts/Roboto-Medium.ttf").unwrap())
        .unwrap();

    // ===============
    // PART OF DEBTORS
    // ===============

    y = Mm(205.0);
    let pos_x: Vec<f32> = vec![5.0, 40.0, 80.0, 100.0, 140.0];
    let header: Vec<&str> = vec!["ID_Devedor", "|Nome", "|Dívida", "|Total Gasto", "|Status"];

    for col in 0..5 {
        current_layer.use_text(header[col], 12.0, Mm(pos_x[col]), y, &font);
    }

    x = Mm(0.2);
    y -= Mm(2.0);
    for _ in 0..300 {
        current_layer.use_text("____", 12.0, x, y, &font);
        x += Mm(2.0);
    }

    let debtors = data.filter_debtors();

    for debtor in debtors {
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

        for col in 0..5 {
            let line = vec![
                format!("{}", debtor.get_id()),
                format!("| {:.18}", debtor.get_name()),
                format!("| {:.2}", debtor.get_debt()),
                format!("| {:.2}", debtor.get_value()),
                format!("| {}", debtor.get_status()),
            ];

            current_layer.use_text(&line[col], 12.0, Mm(pos_x[col]), y, &font);
        }

        for _ in 0..300 {
            current_layer.use_text("____", 12.0, x, y, &font);
            x += Mm(2.0);
        }
    }

    // ==============
    // PART OF COUNTS
    // ==============

    x = Mm(297.0);
    y = Mm(210.0);
    page_count += 1;
    let (page, layer) = doc.add_page(x, y, format!("Página {}", page_count));
    current_layer = doc.get_page(page).get_layer(layer);

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
