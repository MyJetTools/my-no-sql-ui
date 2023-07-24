use flurl::IntoFlUrl;
use serde::Deserialize;

use super::BASE_URL;

pub async fn get_list_of_partitions(table_name: &str) -> ListOfPartitions {
    let resp = BASE_URL
        .append_path_segment("Partitions")
        .append_query_param("tableName", Some(table_name))
        .get()
        .await;

    match resp {
        Ok(data) => {
            let result = data.receive_body().await.unwrap();

            return serde_json::from_slice(&result).unwrap();
        }
        Err(_err) => {
            panic!("Login failed - you need a login server running on localhost:8080.")
        }
    };
}

#[derive(Deserialize)]
pub struct ListOfPartitions {
    pub amount: usize,
    pub data: Vec<String>,
}
