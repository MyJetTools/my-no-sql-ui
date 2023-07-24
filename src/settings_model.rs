use serde::Deserialize;

#[derive(my_settings_reader::SettingsModel, Debug, Deserialize)]
pub struct SettingsModel {
    pub servers: Vec<MyNoSqlConfig>,
}

#[derive(Debug, Deserialize)]
pub struct MyNoSqlConfig {
    pub url: String,
    pub cert_location: Option<String>,
}
