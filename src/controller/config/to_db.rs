use deadpool_postgres::Config;
use deadpool_postgres::ManagerConfig; 
use deadpool_postgres::RecyclingMethod; 
use crate::var; 
use crate::Error;

#[allow(dead_code)]
pub fn get_config() -> Result<Config, Box<dyn Error>> {
    let mut config = Config::new();

    config.user = Some(var("DB_USER").unwrap_or_else(|_| "postgres".into()));
    config.password = Some(var("DB_PASSWORD").unwrap_or_else(|_| "postgres".into()));
    config.dbname = Some(var("DB_NAME").unwrap_or_else(|_| "Esmeralda".into()));
    config.host = Some(var("DB_HOSTNAME").unwrap_or_else(|_| "localhost".into()));
    config.manager = Some(ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    });

    Ok(config)
}