use chrono::Local;
use clap::parser::RawValues;
use clap::{Arg, Command, Subcommand};
use rusqlite::{params, Connection, Result};
use std::fs;
use std::process;

mod backup;
mod comands;
mod create;
mod delete;
mod restore;
mod update;

fn main() {
    let matches = Command::new("GerenciadorDB")
        .about("Ferramenta para gerenciar o banco de dados Esmeralda")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("create")
                .about("Cria o banco de dados")
                .arg(
                    Arg::new("version")
                        .short('v')
                        .long("version")
                        .help("Cria o banco de dados."),
                )
                .arg(
                    Arg::new("path")
                        .short('p')
                        .long("path")
                        .help("Define caminho do banco de dados"),
                ),
        )
        .subcommand(
            Command::new("update")
                .about("Atualiza o banco de dados para uma nova versão")
                .arg(
                    Arg::new("version")
                        .short('v')
                        .long("version")
                        .help("Versão do banco de dados")
                        .required(true),
                ),
        )
        .subcommand(
            Command::new("backup")
                .about("Faz backup do banco de dados")
                .arg(
                    Arg::new("path")
                        .short('p')
                        .long("path")
                        .help("Caminho para salvar o backup")
                        .required(true),
                ),
        )
        .subcommand(
            Command::new("restore")
                .about("Restaura o banco de dados a partir de um backup")
                .arg(
                    Arg::new("backup_file")
                        .short('b')
                        .long("backup_file")
                        .help("Arquivo de backup a ser restaurado")
                        .required(true),
                ),
        )
        .subcommand(Command::new("delete").about("Deleta o banco de dados"))
        .get_matches();

    let conn = Connection::open(match matches.subcommand() {
        Some((_, sub_m)) => {
            let path = sub_m
                .get_raw("path")
                .unwrap_or_default()
                .last()
                .unwrap_or_default()
                .to_str()
                .unwrap();
            path
        }
        _ => "./database.db",
    })
    .unwrap_or_else(|_| {
        println!("Erro ao abrir o banco de dados.");
        process::exit(1);
    });

    match matches.subcommand() {
        Some(("create", sub_m)) => {
            let last_version = sub_m
                .get_raw("version")
                .unwrap_or_default()
                .last()
                .unwrap_or_default()
                .to_str()
                .unwrap();
            create::create_database(&conn, String::from(last_version)).unwrap();
        }
        Some(("update", sub_m)) => {
            let version = sub_m
                .get_raw("version")
                .unwrap()
                .last()
                .unwrap()
                .to_str()
                .unwrap();
            update::update_database(&conn, version).unwrap();
        }
        Some(("backup", sub_m)) => {
            let path = sub_m
                .get_raw("path")
                .unwrap()
                .last()
                .unwrap()
                .to_str()
                .unwrap();
            backup::backup_database(path).unwrap();
        }
        Some(("restore", sub_m)) => {
            let backup_file = sub_m
                .get_raw("backup_file")
                .unwrap()
                .last()
                .unwrap()
                .to_str()
                .unwrap();
            restore::restore_database(backup_file).unwrap();
        }
        Some(("delete", _)) => {
            delete::delete_database().unwrap();
        }
        _ => unreachable!(),
    }
}
