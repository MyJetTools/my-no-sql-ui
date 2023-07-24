use my_json::json_reader::{array_parser::JsonArrayIterator, JsonFirstLineReader};

use super::BASE_URL;
pub async fn get_list_of_rows(table_name: &str, partition_key: &str) -> Vec<Vec<(String, String)>> {
    let resp = reqwest::Client::new()
        .get(format!(
            "{BASE_URL}/Row?tableName={table_name}&partitionKey={partition_key}"
        ))
        .send()
        .await;

    if let Err(err) = &resp {
        println!("Error: {}", err);
    }

    let resp = resp.unwrap();

    let bytes: Vec<u8> = resp.bytes().await.unwrap().into();

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
