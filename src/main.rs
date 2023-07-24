use crate::states::*;
use dioxus::prelude::*;
use elements::*;

mod actions;
mod elements;
mod settings_model;
mod states;

#[tokio::main]
async fn main() {
    dioxus_desktop::launch_cfg(
        app,
        dioxus_desktop::Config::new().with_custom_head(
            r#"
            <style>
            body{
                margin: 0;
                padding: 0;
                font-family: 'Tahoma', sans-serif;
            }

            #main{
                overflow: hidden;
            }

            .btn-menu{
                display: block;
                width:100%;
            }

            .table-item{
                padding:5px;
                color: gray;
                cursor: pointer;
            }

            .selected{
              color: black !important;
              border-radius: 5px;
              background-color: lightgray;
              box-shadow: 0 0 3px lightgray;
            }

            </style>"#
                .to_string(),
        ),
    );
}

fn app(cx: Scope) -> Element {
    use_shared_state_provider(cx, || GlobalState::ReadingSettings);

    use_shared_state_provider(cx, || RightPanelState::new());

    let global_state = use_shared_state::<GlobalState>(cx).unwrap();

    let need_to_read_settings = {
        let global_state = global_state.read();
        global_state.is_reading_settings()
    };

    if need_to_read_settings {
        let global_state_owned = global_state.to_owned();

        cx.spawn(async move {
            let result = settings_model::SettingsModel::read_from_file(".my-no-sql-ui".to_string())
                .await
                .unwrap();

            let mut settings = global_state_owned.write();
            *settings = GlobalState::Active(ActiveState {
                active_config: result.servers.get(0).cloned(),
                settings: result,

                selected_table: None,
                tables: None,
            });
        });
    }

    match global_state.read().as_ref() {
        GlobalState::ReadingSettings => render! { div { "Reading settings..." } },
        GlobalState::Active(_) => render! { working_app {} },
        GlobalState::Error(err) => render! { div { "{err}" } },
    }
}

fn working_app(cx: Scope) -> Element {
    actions::get_list_of_tables(&cx);

    let global_state = use_shared_state::<GlobalState>(cx).unwrap();

    let right_panel_state = use_shared_state::<RightPanelState>(cx).unwrap();

    let input_style = r"height: 100vh; text-align: center;";

    /*
       let main_routine = use_coroutine(cx, |mut rx: UnboundedReceiver<UiCommand>| async move {
           while let Some(msg) = rx.next().await {
               msg.handle_event(&global_state_owned, &right_panel_state_owned)
                   .await;
           }
       });
    */
    render!(
        div { style: "{input_style}",
            table { style: "height: 100vh; width:100%;",
                tr {
                    td { style: "width: 20%; height:100vh;  vertical-align: top; text-align: left;",
                        div { style: " height: 100vh;  overflow-y: auto;",
                            left_panel {
                                on_table_selected: |selected_table| {
                                    load_partitions(cx, global_state, right_panel_state, selected_table);
                                }
                            }
                        }
                    }

                    td { style: "width: 80%; height:100vh; vertical-align: top; text-align: left;",
                        div { style: " height: 100vh;  overflow-y: auto;",
                            right_part {
                                on_partition_select: |partition_key| {
                                    load_rows(cx, global_state, right_panel_state, partition_key);
                                }
                            }
                        }
                    }
                }
            }
        }
    )
}

fn load_partitions(
    cx: &Scoped,
    global_state: &UseSharedState<GlobalState>,
    right_panel_state: &UseSharedState<RightPanelState>,
    table_name: String,
) {
    right_panel_state.write().set_loading();

    global_state.write().set_selected_table(table_name.clone());

    let active_config = global_state.read().unwrap_active_config();

    let right_panel_state = right_panel_state.to_owned();

    cx.spawn(async move {
        let result = tokio::spawn(async move {
            let result =
                actions::get_list_of_partitions(active_config.clone(), table_name.clone()).await;

            if let Err(err) = &result {
                return RightPanelState::Error(err.to_string());
            }

            let mut result = result.unwrap();

            if result.data.len() == 1 {
                let partition_key = result.data.remove(0);
                let rows = actions::get_list_of_rows(
                    active_config,
                    table_name.clone(),
                    partition_key.clone(),
                )
                .await;

                if let Err(err) = &rows {
                    return RightPanelState::Error(err.to_string());
                }

                let rows = rows.unwrap();

                return RightPanelState::LoadedRows(LoadedRows {
                    loading: false,
                    partition_key: partition_key.to_string(),
                    partitions: vec![partition_key],
                    rows,
                });
            }

            if result.data.len() > 1 {
                return RightPanelState::LoadedPartitions(LoadedPartitions {
                    loading: false,
                    table_name: table_name,
                    partitions: result.data,
                    amount: result.amount,
                });
            } else {
                RightPanelState::NoPartitions(table_name)
            }
        })
        .await
        .unwrap();

        let mut right_panel_state = right_panel_state.write();
        *right_panel_state = result;
    });
}

fn load_rows(
    cx: &Scoped,
    global_state: &UseSharedState<GlobalState>,
    right_panel_state: &UseSharedState<RightPanelState>,
    partition_key: String,
) {
    right_panel_state.write().set_loading();
    let active_config = global_state.read().unwrap_active_config();
    let table_name = global_state.read().get_selected_table().unwrap().clone();
    let right_panel_state = right_panel_state.to_owned();
    cx.spawn(async move {
        let rows = tokio::spawn(actions::get_list_of_rows(
            active_config,
            table_name.clone(),
            partition_key.clone(),
        ))
        .await
        .unwrap();

        match rows {
            Ok(rows) => {
                let mut right_panel_state = right_panel_state.write();
                right_panel_state.promote_to_loaded_rows(partition_key, rows);
            }
            Err(err) => {
                let mut right_panel_state = right_panel_state.write();
                *right_panel_state = RightPanelState::Error(err);
            }
        }
    });
}
