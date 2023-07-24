pub struct LoadedRows {
    pub loading: bool,
    pub partition_key: String,
    pub partitions: Vec<String>,
    pub rows: Vec<Vec<(String, String)>>,
}

impl LoadedRows {
    pub fn get_amount(&self) -> usize {
        self.rows.len()
    }

    pub fn get_list_of_headers(&self) -> Vec<String> {
        let mut headers = Vec::new();

        for record in &self.rows {
            for (name, _) in record {
                if !headers.contains(name) {
                    if name == "PartitionKey" {
                        headers.insert(0, name.clone());
                    } else if name == "RowKey" {
                        if headers.len() > 0 {
                            headers.insert(1, name.clone());
                        } else {
                            headers.push(name.clone());
                        }
                    } else {
                        headers.push(name.clone());
                    }
                }
            }
        }

        headers
    }

    pub fn get_values(&self, no: usize, headers: &[String]) -> Vec<Option<&str>> {
        let record = self.rows.get(no).unwrap();

        let mut result = Vec::new();
        for header in headers {
            result.push(find_value(record.as_slice(), header));
        }

        result
    }
}

fn find_value<'s>(records: &'s [(String, String)], header_name: &str) -> Option<&'s str> {
    for (name, value) in records {
        if name == header_name {
            return Some(value);
        }
    }

    None
}
