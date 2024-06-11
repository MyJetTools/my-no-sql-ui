use dioxus::prelude::*;

use serde::*;

use crate::states::{GlobalState, TablesList};

pub fn get_list_of_tables() {
    let global_state = use_shared_state::<GlobalState>(cx).unwrap();
    let tables_list = use_shared_state::<TablesList>(cx).unwrap();

    if tables_list.read().tables_are_loaded() {
        return;
    }

    let tables_list = tables_list.to_owned();

    let active_config = global_state.read().unwrap_active_config();

    cx.spawn(async move {
        let names = tokio::spawn(async move {
            let resp = active_config
                .get_fl_url()
                .await
                .append_path_segment("Tables")
                .append_path_segment("List")
                .get()
                .await;

            match resp {
                Ok(mut data) => {
                    let status_code = data.get_status_code();
                    if status_code != 200 {
                        let result = data.receive_body().await.unwrap();
                        panic!(
                            "Code: {}. Msg: {}",
                            status_code,
                            String::from_utf8(result).unwrap()
                        );
                    }
                    let result: Vec<Table> = data.get_json().await.unwrap();

                    let names: Vec<String> = result.into_iter().map(|table| table.name).collect();
                    Ok(names)
                }
                Err(err) => Err(format!("Can not retrieve tables from server {:?}", err)),
            }
        })
        .await;

        if let Err(err) = &names {
            tables_list.write().set_error(format!(
                "Can not retrieve tables from server: Err:{:?}",
                err
            ));
            return;
        }

        match names.unwrap() {
            Ok(names) => {
                tables_list.write().set_loaded_tables(names);
            }
            Err(err) => {
                tables_list.write().set_error(err);
            }
        }
    });
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Table {
    pub name: String,
}
