use dioxus::prelude::{use_shared_state, Scope};
use flurl::IntoFlUrl;
use serde::*;

use super::BASE_URL;
use crate::states::Tables;

pub fn get_list_of_tables<'s>(cx: &'s Scope<'s>) {
    let tables = use_shared_state::<Tables>(cx).unwrap();

    if tables.read().names.is_some() {
        return;
    }

    cx.spawn({
        let tables = tables.to_owned();

        async move {
            let resp = BASE_URL
                .append_path_segment("Tables")
                .append_path_segment("List")
                .get()
                .await;

            match resp {
                Ok(data) => {
                    let result = data.receive_body().await.unwrap();

                    let result: Vec<Table> = serde_json::from_slice(&result).unwrap();

                    let names: Vec<String> = result.into_iter().map(|table| table.name).collect();

                    tables.write().names = names.into();
                }
                Err(_err) => {
                    println!("Login failed - you need a login server running on localhost:8080.")
                }
            }
        }
    });
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Table {
    pub name: String,
}
