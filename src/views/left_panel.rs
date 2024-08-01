use dioxus::prelude::*;
use serde::*;

use crate::states::*;

#[component]
pub fn LeftPanel() -> Element {
    let tables_list = consume_context::<Signal<TablesList>>();

    let (tables, err, selected_table, tables_are_loading, filter) = {
        let read_access = tables_list.read();

        let selected_table = read_access.get_selected_table();
        let tables = read_access.get_tables();
        let err = read_access.get_err();
        let loading = read_access.loading;

        (
            tables,
            err,
            selected_table,
            loading,
            read_access.filter.clone(),
        )
    };

    if let Some(err) = err {
        return rsx! {
            div { class: "alert alert-danger", "{err}" }
        };
    }

    let env = consume_context::<Signal<EnvListState>>()
        .read()
        .get_selected_env();

    if env.is_none() {
        return None;
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

        return None;
    }

    let tables = tables.unwrap();

    let table_names = tables
        .into_iter()
        .filter(|itm| {
            if filter.is_empty() {
                return true;
            }

            itm.name.contains(&filter)
        })
        .map(|table| {
            let selected = if let Some(selected_table) = &selected_table {
                selected_table == &table.name
            } else {
                false
            };

            let mut content = Vec::new();

            content.push(rsx! { "{table.name}" });

            if let Some(persist) = table.persist {
                if !persist {
                    content.push(rsx! {
                        span { class: "badge text-bg-danger", "NoPersist" }
                    });
                }
            }

            if let Some(max_partitions_amount) = table.max_partitions_amount {
                content.push(rsx! {
                    span { class: "badge text-bg-warning", "MP:{max_partitions_amount}" }
                });
            };

            if let Some(max_rows_per_partition_amount) = table.max_rows_per_partition_amount {
                content.push(rsx! {
                    span { class: "badge text-bg-info", "MR:{max_rows_per_partition_amount}" }
                });
            };

            if selected {
                rsx! {
                    div { class: "table-item selected", {content.into_iter()} }
                }
            } else {
                let env = env.clone();
                rsx! {
                    div {
                        class: "table-item",
                        onclick: move |_| {
                            consume_context::<Signal<TablesList>>()
                                .write()
                                .set_selected_table(table.name.as_str().to_string());
                            consume_context::<Signal<RightPanelState>>()
                                .write()
                                .load_partitions(env.clone(), table.name.clone());
                        },
                        {content.into_iter()}
                    }
                }
            }
        });

    rsx! {

        input {
            class: "form-control",
            style: "margin-top: 5px; margin-bottom: 5px;",
            placeholder: "Filter",
            value: filter,
            oninput: move |evt| {
                let value = evt.value();
                consume_context::<Signal<TablesList>>().write().filter = value;
            }
        }
        {table_names}
    }
}

#[server]
async fn get_tables(env: String) -> Result<Vec<TableJsonModel>, ServerFnError> {
    let fl_url = crate::APP_CTX.get_fl_url(env.as_str()).await;

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
            Ok(result)
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
    pub persist: Option<bool>,

    #[serde(rename = "maxPartitionsAmount")]
    pub max_partitions_amount: Option<usize>,

    #[serde(rename = "maxRowsPerPartitionAmount")]
    pub max_rows_per_partition_amount: Option<usize>,
}
