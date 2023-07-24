use std::collections::HashMap;

use flurl::ClientCertificate;
use rust_extensions::StopWatch;
use serde::Deserialize;
use tokio::sync::RwLock;

#[derive(my_settings_reader::SettingsModel, Deserialize, Clone)]
pub struct SettingsModel {
    pub servers: Vec<MyNoSqlConfig>,
}

#[derive(Deserialize, Clone)]
pub struct MyNoSqlConfig {
    pub url: String,
    pub name: String,
    pub cert_location: Option<String>,
    pub cert_password: Option<String>,
}

lazy_static::lazy_static! {
    pub static ref SETTINGS_MODEL: RwLock<HashMap<String, ClientCertificate>> = RwLock::new(HashMap::new());
}

impl MyNoSqlConfig {
    pub async fn get_fl_url(&self) -> flurl::FlUrl {
        if let Some(cert_location) = &self.cert_location {
            {
                let read_access = SETTINGS_MODEL.read().await;
                if let Some(value) = read_access.get(cert_location) {
                    return flurl::FlUrl::new_with_timeout(
                        self.url.as_str(),
                        std::time::Duration::from_secs(3),
                    )
                    .with_client_certificate(value.clone());
                }
            }

            if let Some(cert_password) = &self.cert_password {
                let identity = load_cert(cert_location, cert_password).await;

                let mut write_access = SETTINGS_MODEL.write().await;
                write_access.insert(cert_location.to_string(), identity.clone());

                return flurl::FlUrl::new_with_timeout(
                    self.url.as_str(),
                    std::time::Duration::from_secs(3),
                )
                .with_client_certificate(identity);
            }
        }
        flurl::FlUrl::new_with_timeout(self.url.as_str(), std::time::Duration::from_secs(3))
    }
}

async fn load_cert(path: &str, password: &str) -> ClientCertificate {
    let path = if path.starts_with("~") {
        path.replace("~", std::env::var("HOME").unwrap().as_str())
    } else {
        path.to_string()
    };

    let mut sw = StopWatch::new();
    sw.start();
    let content: &Vec<u8> = &tokio::fs::read(path.as_str()).await.unwrap();
    sw.pause();

    let result = ClientCertificate::from_pkcs12(content, password);

    result
}
