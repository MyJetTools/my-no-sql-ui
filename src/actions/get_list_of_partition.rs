use serde::Deserialize;

use crate::settings_model::MyNoSqlConfig;

pub async fn get_list_of_partitions(
    current_config: MyNoSqlConfig,
    table_name: String,
) -> Result<ListOfPartitions, String> {
    let resp = current_config
        .get_fl_url()
        .await
        .append_path_segment("Partitions")
        .append_query_param("tableName", Some(table_name))
        .get()
        .await;

    match resp {
        Ok(mut data) => {
            if data.get_status_code() != 200 {
                let result = data.receive_body().await.unwrap();
                return Err(String::from_utf8(result).unwrap());
            }

            return Ok(data.get_json().await.unwrap());
        }
        Err(err) => {
            return Err(format!("{:?}", err));
        }
    };
}

#[derive(Deserialize)]
pub struct ListOfPartitions {
    pub amount: usize,
    pub data: Vec<String>,
}
