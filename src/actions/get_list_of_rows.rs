use my_json::json_reader::{array_parser::JsonArrayIterator, JsonFirstLineReader};

use crate::settings_model::MyNoSqlConfig;

pub async fn get_list_of_rows(
    current_config: MyNoSqlConfig,
    table_name: String,
    partition_key: String,
) -> Result<Vec<Vec<(String, String)>>, String> {
    let resp = current_config
        .get_fl_url()
        .await
        .append_path_segment("Row")
        .append_query_param("tableName", Some(table_name))
        .append_query_param("partitionKey", Some(partition_key))
        .get()
        .await;

    if let Err(err) = &resp {
        return Err(format!("{:?}", err));
    }

    let resp = resp.unwrap();

    if resp.get_status_code() != 200 {
        let result = resp.receive_body().await.unwrap();
        return Err(String::from_utf8(result).unwrap());
    }

    let bytes: Vec<u8> = resp.receive_body().await.unwrap();

    let mut rows = Vec::new();

    for json_item in JsonArrayIterator::new(bytes.as_slice()) {
        let json_item = json_item.unwrap();

        let mut item = Vec::new();

        for line in JsonFirstLineReader::new(json_item) {
            let line = line.unwrap();

            item.push((
                line.get_name().unwrap().to_string(),
                std::str::from_utf8(&line.data[line.value_start..line.value_end])
                    .unwrap()
                    .to_string(),
            ));
        }

        rows.push(item);
    }

    Ok(rows)
}
