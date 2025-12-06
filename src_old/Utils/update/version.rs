use super::*;

pub fn has_new_version() -> bool {
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
            version.as_str() != format!("v{}", env::var("CARGO_PKG_VERSION").unwrap())
        } else {
            false
        }
    } else {
        false
    }
}
