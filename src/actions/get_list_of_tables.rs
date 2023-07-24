use dioxus::prelude::{use_shared_state, Scope};

use serde::*;

use crate::states::GlobalState;

pub fn get_list_of_tables<'s>(cx: &'s Scope<'s>) {
    let global_state = use_shared_state::<GlobalState>(cx).unwrap();

    if global_state.read().tables_are_loaded() {
        return;
    }

    let global_state = global_state.to_owned();

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
                Ok(data) => {
                    let result = data.receive_body().await.unwrap();

                    let result: Vec<Table> = serde_json::from_slice(&result).unwrap();

                    let names: Vec<String> = result.into_iter().map(|table| table.name).collect();
                    names
                }
                Err(_err) => {
                    panic!("Login failed - you need a login server running on localhost:8080.")
                }
            }
        })
        .await
        .unwrap();

        global_state.write().set_loaded_tables(names);
    });
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Table {
    pub name: String,
}
