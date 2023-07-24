use serde::Deserialize;

use super::BASE_URL;

pub async fn get_list_of_partitions(table_name: &str) -> ListOfPartitions {
    let resp = reqwest::Client::new()
        .get(format!("{BASE_URL}/Partitions?tableName={table_name}"))
        .send()
        .await;

    match resp {
        Ok(data) => {
            let result = data.bytes().await.unwrap();
            let result: Vec<u8> = result.into();

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
