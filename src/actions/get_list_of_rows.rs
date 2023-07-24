use flurl::IntoFlUrl;
use my_json::json_reader::{array_parser::JsonArrayIterator, JsonFirstLineReader};

use super::BASE_URL;
pub async fn get_list_of_rows(table_name: &str, partition_key: &str) -> Vec<Vec<(String, String)>> {
    let resp = BASE_URL
        .append_path_segment("Row")
        .append_query_param("tableName", Some(table_name))
        .append_query_param("partitionKey", Some(partition_key))
        .get()
        .await;

    if let Err(_) = &resp {
        println!("Error getting list of rows");
    }

    let resp = resp.unwrap();

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

    rows
}
