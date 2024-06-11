use crate::settings_model::SettingsModel;

pub struct AppContext;

impl AppContext {
    pub fn new() -> Self {
        AppContext
    }

    pub async fn get_settings(&self) -> SettingsModel {
        SettingsModel::read_from_file(".my-no-sql-ui".to_string())
            .await
            .unwrap()
    }
}
