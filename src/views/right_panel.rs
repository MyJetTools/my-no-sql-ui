use std::rc::Rc;

use dioxus::prelude::*;

use serde::*;

use crate::states::*;

#[component]
pub fn RightPanel() -> Element {
    let mut right_panel_state = consume_context::<Signal<RightPanelState>>();

    let right_panel_read_access = right_panel_state.read();

    if right_panel_read_access.env.is_none() {
        return render_nothing();
    }

    if right_panel_read_access.table_name.is_none() {
        return render_nothing();
    }

    if let Some(err) = right_panel_read_access.error.as_ref() {
        return rsx! {
            div { style: "padding:5px", "{err}" }
        };
    }

    let table_name = right_panel_read_access.unwrap_table_name();

    if right_panel_read_access.partitions.is_none() {
        let mut right_panel_state = right_panel_state.to_owned();
        spawn(async move {
            if right_panel_state.read().loading_partitions {
                return;
            }

            let (env, table_name) = {
                let mut write_access = right_panel_state.write();
                if write_access.loading_partitions {
                    return;
                }
                write_access.set_loading_partitions();

                (write_access.unwrap_env(), write_access.unwrap_table_name())
            };

            let result = load_partitions(env.to_string(), table_name.to_string()).await;

            match result {
                Ok(result) => {
                    right_panel_state.write().set_loaded_partitions(result.data);
                }
                Err(err) => right_panel_state.write().set_error(format!("{:?}", err)),
            }
        });

        return rsx! {
            h3 { "Loading partitions for table: {table_name.as_str()}" }
        };
    }

    let partitions = right_panel_read_access.partitions.clone().unwrap();

    if partitions.len() == 0 {
        let table_name = table_name.clone();
        let msg = format!("No partitions found for table '{table_name}'");
        return rsx! {
            div { style: "padding:5px", {msg} }
        };
    }

    let selected_partition = right_panel_read_access.get_selected_partition();

    if selected_partition.is_none() {
        let partitions = partitions.iter().map(|partition_key| {
            let partition_key = partition_key.clone();

            rsx! {
                div {
                    style: "margin:2px; display: inline-block;  cursor:pointer",
                    class: "btn btn-outline-dark btn-sm",
                    onclick: move |_| {
                        consume_context::<Signal<RightPanelState>>()
                            .write()
                            .select_partition(partition_key.clone());
                    },
                    "{partition_key}"
                }
            }
        });

        return rsx! {
            {partitions}
        };
    }

    let selected_partition = selected_partition.unwrap();

    if right_panel_read_access.loaded_rows.is_none() {
        let mut right_panel_state = right_panel_state.to_owned();
        let selected_partition_spawn = selected_partition.clone();

        spawn(async move {
            if right_panel_state.read().loading_rows {
                return;
            }

            let (env, table_name) = {
                let mut write_access = right_panel_state.write();

                if write_access.loading_rows {
                    return;
                }

                write_access.set_loading_rows();

                (write_access.unwrap_env(), write_access.unwrap_table_name())
            };

            let rows = load_rows(
                env.to_string(),
                table_name.to_string(),
                selected_partition_spawn.to_string(),
            )
            .await;

            match rows {
                Ok(rows) => {
                    right_panel_state.write().set_loaded_rows(rows);
                }
                Err(err) => {
                    right_panel_state.write().set_error(format!("{}", err));
                }
            }
        });

        return rsx! {
            h1 { "Loading rows for partition {selected_partition}" }
        };
    }

    let data = right_panel_read_access.loaded_rows.clone().unwrap();

    let headers = data.get_list_of_headers();
    let amount = data.get_amount();

    let rows_to_render = partitions.iter().map(|itm| {
        rsx! {
            option { value: "{itm}", selected: itm == &selected_partition, "{itm}" }
        }
    });

    let headers_to_render = headers.iter().map(|header| {
        rsx! {
            th { "{header}" }
        }
    });

    let mut table_values_to_render = Vec::with_capacity(amount);

    for no in 0..amount {
        let values = data.get_values(no, &headers);

        let skip_it = if right_panel_read_access.filter_line.len() > 0 {
            let mut skip_it = true;
            for value in &values {
                if let Some(value) = value {
                    if value.contains(right_panel_read_access.filter_line.as_str()) {
                        skip_it = false;
                        break;
                    }
                }
            }

            skip_it
        } else {
            false
        };

        if skip_it {
            continue;
        }

        let values_to_render = values.iter().map(|value|{
            match value{
                Some(value)=>{
                    rsx!{
                        td {
                            div { style: "width:200px; height:100px; overflow-y:auto; overflow-wrap:anywhere",
                                "{value}"
                            }
                        }
                    }

                },
                None=>{
                    rsx!{
                        td {}
                    }
                }
            }
        });

        table_values_to_render.push(rsx! {
            tr { {values_to_render} }
        });
    }

    rsx! {

        div { style: "overflow-x:scroll",
            span { "Partition Key:" }
            select {
                class: "form-control",
                style: "width: 200px; display:inline; font-size:12px;",
                onchange: move |evn| {
                    let value = evn.value();
                    right_panel_state.write().select_partition(Rc::new(value));
                },

                {rows_to_render}
            }

            input {
                class: "form-control",
                style: "width: 200px; display:inline; font-size:12px;",
                placeholder: "Filter",
                oninput: move |evn| {
                    right_panel_state.write().filter_line = evn.value().trim().to_string();
                }
            }

            div { style: "height: calc(var(--app-height) - 30px); overflow-y:auto",
                table {
                    style: "width:auto;font-size:10px;min-width: 100%;",
                    class: "table table-bordered  table-sm",
                    thead {
                        class: "table-light",
                        style: "position: sticky;top: 0;background-color: lightgray;",
                        {headers_to_render}
                    }

                    {table_values_to_render.into_iter()}
                }
            }
        }
    }
}

fn render_nothing() -> Element {
    rsx! {
        div {}
    }
}

#[server]
async fn load_partitions(
    env: String,
    table_name: String,
) -> Result<ListOfPartitionsJsonModel, ServerFnError> {
    let resp = crate::APP_CTX
        .get_settings()
        .await
        .get_my_no_sql_config(env.as_str())
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
                let err_msg = String::from_utf8(result).unwrap();
                return Err(ServerFnError::new(err_msg));
            }

            return Ok(data.get_json().await.unwrap());
        }
        Err(err) => {
            let err_msg = format!("{:?}", err);
            return Err(ServerFnError::new(err_msg));
        }
    };
}

#[derive(Deserialize, Serialize)]
pub struct ListOfPartitionsJsonModel {
    pub amount: usize,
    pub data: Vec<String>,
}

#[server]
async fn load_rows(
    env: String,
    table_name: String,
    partition_key: String,
) -> Result<Vec<Vec<(String, String)>>, ServerFnError> {
    use my_json::json_reader::array_iterator::JsonArrayIterator;
    use rust_extensions::array_of_bytes_iterator::SliceIterator;

    let resp = crate::APP_CTX
        .get_settings()
        .await
        .get_my_no_sql_config(env.as_str())
        .get_fl_url()
        .await
        .append_path_segment("Row")
        .append_query_param("tableName", Some(table_name))
        .append_query_param("partitionKey", Some(partition_key))
        .get()
        .await;

    if let Err(err) = &resp {
        let err = format!("{:?}", err);
        return Err(ServerFnError::new(err));
    }

    let resp = resp.unwrap();

    if resp.get_status_code() != 200 {
        let result = resp.receive_body().await.unwrap();
        let err = String::from_utf8(result).unwrap();
        return Err(ServerFnError::new(err));
    }

    let bytes: Vec<u8> = resp.receive_body().await.unwrap();

    let mut rows = Vec::new();

    let slice_iterator = SliceIterator::new(bytes.as_slice());

    let mut json_array_iterator = JsonArrayIterator::new(slice_iterator);

    while let Some(json_item) = json_array_iterator.get_next() {
        let json_item = json_item.unwrap();

        let mut item = Vec::new();

        let mut json = json_item.unwrap_as_object(&json_array_iterator).unwrap();

        while let Some(line) = json.get_next() {
            let line = line.unwrap();

            item.push((
                line.name.as_str(&json).unwrap().to_string(),
                match line.value.as_str(&json) {
                    Some(value) => value.to_string(),
                    None => "Null".to_string(),
                },
            ));
        }

        rows.push(item);
    }

    Ok(rows)
}
