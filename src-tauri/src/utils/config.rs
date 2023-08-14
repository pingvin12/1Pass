use config_file::FromConfigFile;
use mockall::lazy_static;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub connection: String,
}
lazy_static! {
    pub static ref CONFIG: Config = get_config();
}

fn get_config() -> Config {
    let config: Config = Config::from_config_file("C:\\Users\\fenye\\source\\repos\\1Pass\\src-tauri\\config.toml").unwrap();

    config
}