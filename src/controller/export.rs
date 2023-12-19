use std::fs::File;
use std::fs::create_dir_all;
use std::io::Write;

use crate::prelude::structs::InterfaceInfo;

async fn mkdir(path: &str) -> File{
    let mut new_path: String = String::new();

    let paths: Vec<&str> = path.split('/').collect();

    for i in 0..paths.len() - 1{
        new_path.push_str(paths[i]);
        new_path.push('/');
    }

    create_dir_all(new_path).unwrap();

    let file = match File::open(path){
        Ok(file) => file,
        Err(_) => File::create(path).unwrap(),
    };

    file
}

pub async fn export_csv( path: &str, data: InterfaceInfo) -> Result<(), String>{
    let data = data.list;

    let mut file: File = mkdir(path).await;

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
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}