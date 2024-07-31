use std::sync::Arc;

use flurl::FlUrl;
use my_settings_reader::SettingsReader;

use crate::settings_model::SettingsModel;

pub struct AppContext {
    ssh_pool: Arc<my_ssh::SshSessionsPool>,
    pub settings_read: SettingsReader<SettingsModel>,
}

impl AppContext {
    pub fn new() -> Self {
        AppContext {
            ssh_pool: Arc::new(my_ssh::SshSessionsPool::new()),
            settings_read: SettingsReader::new(".my_no_sql_ui"),
        }
    }

    pub async fn get_fl_url(&self, env: &str) -> FlUrl {
        let settings_model = self.settings_read.get_settings().await;

        settings_model
            .get_my_no_sql_config(env)
            .get_fl_url(settings_model.ssh_credentials.as_ref(), &self.ssh_pool)
            .await
    }
}
