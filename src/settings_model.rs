use std::time::Duration;

use flurl::my_tls::ClientCertificate;
use serde::Deserialize;

#[derive(my_settings_reader::SettingsModel, Deserialize, Clone)]
pub struct SettingsModel {
    pub servers: Vec<MyNoSqlConfig>,
}

impl SettingsModel {
    pub fn get_my_no_sql_config(&self, env_name: &str) -> MyNoSqlConfig {
        self.servers
            .iter()
            .find(|x| x.name == env_name)
            .unwrap()
            .clone()
    }
}

#[derive(Deserialize, Clone)]
pub struct MyNoSqlConfig {
    pub url: String,
    pub name: String,
    pub server_name: Option<String>,
    pub cert_location: Option<String>,
    pub cert_password: Option<String>,
}

#[derive(Deserialize, Clone)]
pub struct SshConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
}

impl MyNoSqlConfig {
    pub async fn get_fl_url(&self) -> flurl::FlUrl {
        if let Some(cert_location) = &self.cert_location {
            if let Some(cert_password) = &self.cert_password {
                let cert_location = rust_extensions::file_utils::format_path(cert_location);

                let client_certificate =
                    ClientCertificate::from_pks12_file(cert_location.as_str(), cert_password).await;

                let result = flurl::FlUrl::new(self.url.as_str())
                    .set_timeout(Duration::from_secs(3))
                    .with_client_certificate(client_certificate);

                if let Some(server_name) = &self.server_name {
                    return result.set_tls_domain(server_name.to_string());
                }

                return result;
            }
        }
        flurl::FlUrl::new(self.url.as_str()).set_timeout(Duration::from_secs(3))
    }
}
