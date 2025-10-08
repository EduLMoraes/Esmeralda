use crate::{
    env,
    regex::Regex,
    std::process::{Command, Stdio},
};
use reqwest::Client;
use std::{error::Error, fs::File, io::Write, path::Path};
use tokio;
use zip::read::ZipArchive;

mod download;
mod unzip;
use download::*;
use unzip::*;
mod version;
use version::has_new_version;

#[allow(dead_code)]
pub fn update_sys() {
    let output = Command::new("wget")
        .arg("-q")
        .arg("--server-response")
        .arg("-O-")
        .arg("https://github.com/EduLMoraes/Esmeralda/releases/latest/download/sources.zip")
        .stderr(Stdio::piped())
        .output()
        .unwrap();

    let stderr = String::from_utf8_lossy(&output.stderr);

    let re = Regex::new(r"releases/download/(v[0-9.]+)/").unwrap();

    if let Some(caps) = re.captures(&stderr) {
        if let Some(version) = caps.get(1) {
            if has_new_version() {
                tracing::error!("Versão mais recente: {}", version.as_str());

                let zip_file_path = format!("{}sources.zip", env::var("UPDT_PATH").unwrap());
                let output_directory = env::var("UPDT_PATH").unwrap().to_string();

                let _ = download_file(
                    "https://github.com/EduLMoraes/Esmeralda/releases/latest/download/esmeralda",
                    &format!("{}esmeralda", &output_directory),
                );
                let _ = download_file(
                    "https://github.com/EduLMoraes/Esmeralda/releases/latest/download/sources.zip",
                    &zip_file_path,
                );

                if let Err(e) = unzip(&zip_file_path, &output_directory) {
                    tracing::error!("Failed to unzip file: {}", e);
                }
            }
        }
        else {
            tracing::error!("Versão não encontrada");
        }
    }
    else {
        tracing::error!("URL de redirecionamento não encontrada");
    }
}
