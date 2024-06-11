use std::rc::Rc;

use dioxus::prelude::*;
use serde::*;

use crate::states::*;

use super::*;

#[component]
pub fn LeftPanel() -> Element {
    let tables_list = consume_context::<Signal<TablesList>>();

    let (tables, err, selected_table, tables_are_loading) = {
        let read_access = tables_list.read();

        let selected_table = read_access.get_selected_table();
        let tables = read_access.get_tables();
        let err = read_access.get_err();
        let loading = read_access.loading;

        (tables, err, selected_table, loading)
    };

    if let Some(err) = err {
        return rsx! {

            div { id: "left-panel", style: "padding:5px",
                EnvList {}
                div { class: "alert alert-danger", "{err}" }
            }
        };
    }

    let env = consume_context::<Signal<EnvListState>>()
        .read()
        .selected_env
        .clone();

    if env.is_none() {
        return rsx! {
            div { id: "left-panel", style: "padding:5px", EnvList {} }
        };
    }

    let env = env.unwrap();

    if tables.is_none() {
        if !tables_are_loading {
            let mut tables_list = tables_list.to_owned();

            spawn(async move {
                let tables = get_tables(env.to_string()).await;

                match tables {
                    Ok(tables) => {
                        tables_list.write().set_loaded_tables(tables);
                    }
                    Err(err) => {
                        tables_list.write().set_error(err.to_string());
                    }
                }
            });
        }

        return rsx! {
            div { id: "left-panel", style: "padding:5px", EnvList {} }
        };
    }

    let tables = tables.unwrap();

    let table_names = tables.into_iter().map(|name| {
        let name = Rc::new(name);

        let selected = if let Some(selected_table) = &selected_table {
            selected_table == name.as_ref()
        } else {
            false
        };

        if selected {
            rsx! {
                div { class: "table-item selected", "{name}" }
            }
        } else {
            let env = env.clone();
            rsx! {
                div {
                    class: "table-item",
                    onclick: move |_| {
                        consume_context::<Signal<TablesList>>()
                            .write()
                            .set_selected_table(name.as_str().to_string());
                        consume_context::<Signal<RightPanelState>>()
                            .write()
                            .load_partitions(env.clone(), name.clone());
                    },
                    "{name}"
                }
            }
        }
    });

    rsx! {

        div { id: "left-panel", style: "padding:5px",
            EnvList {}
            {table_names}
        }
    }
}

#[server]
async fn get_tables(env: String) -> Result<Vec<String>, ServerFnError> {
    let fl_url = crate::APP_CTX
        .get_settings()
        .await
        .get_my_no_sql_config(env.as_str())
        .get_fl_url()
        .await;

    let response = fl_url
        .append_path_segment("Tables")
        .append_path_segment("List")
        .get()
        .await;

    match response {
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
            let result: Vec<TableJsonModel> = data.get_json().await.unwrap();

            let names: Vec<String> = result.into_iter().map(|table| table.name).collect();
            Ok(names)
        }
        Err(err) => Err(ServerFnError::new(format!(
            "Can not retrieve tables from server {:?}",
            err
        ))),
    }
}
#[derive(Clone, Serialize, Deserialize)]
pub struct TableJsonModel {
    pub name: String,
}
