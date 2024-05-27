use super::*;

pub fn download_file(url: &str, file_path: &str) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let rnt = tokio::runtime::Runtime::new().unwrap();
    let response = rnt.block_on(client.get(url).send())?;

    if !response.status().is_success() {
        return Err(format!("Falha ao baixar arquivo: HTTP {}", response.status()).into());
    }

    let mut dest = File::create(file_path)?;
    dest.write_all(&rnt.block_on(response.bytes()).unwrap());

    Ok(())
}
