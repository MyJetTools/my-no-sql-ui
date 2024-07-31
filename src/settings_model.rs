use std::{collections::HashMap, sync::Arc, time::Duration};

use flurl::my_tls::ClientCertificate;
use my_ssh::SshCredentialsSettingsModel;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct SettingsModel {
    pub envs: Vec<MyNoSqlConfig>,
    pub ssh_credentials: Option<HashMap<String, SshCredentialsSettingsModel>>,
}

impl SettingsModel {
    pub fn get_my_no_sql_config(&self, env_name: &str) -> MyNoSqlConfig {
        self.envs
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

impl MyNoSqlConfig {
    pub async fn get_fl_url(
        &self,
        ssh_credentials: Option<&HashMap<String, SshCredentialsSettingsModel>>,
        ssh_pool: &Arc<my_ssh::SshSessionsPool>,
    ) -> flurl::FlUrl {
        let settings = my_ssh::OverSshConnectionSettings::parse(&self.url, ssh_credentials).await;

        if let Some(ssh_creds) = settings.ssh_credentials {
            return flurl::FlUrl::new(settings.url.as_str())
                .set_timeout(Duration::from_secs(3))
                .set_ssh_credentials(Arc::new(ssh_creds))
                .set_ssh_sessions_pool(ssh_pool.clone());
        }

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
