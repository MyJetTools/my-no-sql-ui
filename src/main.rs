#![allow(non_snake_case)]

use crate::states::*;
use dioxus::prelude::*;
use views::*;

//#[cfg(feature = "server")]
//mod actions;
#[cfg(feature = "server")]
mod app_ctx;
#[cfg(feature = "server")]
mod settings_model;
mod states;
mod views;

#[cfg(feature = "server")]
lazy_static::lazy_static! {
    pub static ref APP_CTX: crate::app_ctx::AppContext = {
        crate::app_ctx::AppContext::new()
    };
}

fn main() {
    let cfg = dioxus::fullstack::Config::new();

    #[cfg(feature = "server")]
    let cfg = cfg.addr(([0, 0, 0, 0], 9001));

    LaunchBuilder::fullstack().with_cfg(cfg).launch(app)
}

fn app() -> Element {
    use_context_provider(|| Signal::new(EnvListState::new()));
    use_context_provider(|| Signal::new(RightPanelState::new()));
    use_context_provider(|| Signal::new(TablesList::new()));

    rsx! {
        LeftPanel {}
        RightPanel {}
    }

    /*
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
            });
        });
    }

    match global_state.read().as_ref() {
        GlobalState::ReadingSettings => render! { div { "Reading settings..." } },
        GlobalState::Active(_) => render! { working_app {} },
    }
     */
}

/*
fn working_app() -> Element {
    actions::get_list_of_tables(&cx);
    let right_panel = use_shared_state::<RightPanelState>(cx).unwrap();
    render!(
        div { id: "main-wrapper",
            table { style: "height: var(--working-area-height); width:100%;",
                tr {
                    td { style: "vertical-align: top; padding:0;",
                        div { id: "left-panel",
                            left_panel {
                                on_table_selected: |selected_table| {
                                    load_partitions(cx, selected_table);
                                }
                            }
                        }
                    }

                    td { style: "vertical-align: top; padding:0;",
                        div { id: "right-panel",
                            right_part {
                                on_partition_select: |partition_key| {
                                    load_rows(cx, partition_key, right_panel);
                                }
                            }
                        }
                    }
                }
            }
        }
        div { id: "top-panel", configuration_panel {} }
    )
}

fn configuration_panel(cx: Scope) -> Element {
    let global_state = use_shared_state::<GlobalState>(cx).unwrap();

    let tables_list = use_shared_state::<TablesList>(cx).unwrap();
    let right_panel = use_shared_state::<RightPanelState>(cx).unwrap();

    let (settings, active_config) = {
        let read = global_state.read();
        let active_state = read.unwrap_as_active();
        let settings = active_state.settings.clone();
        let active_config = active_state.active_config.clone();
        (settings, active_config)
    };

    render! {

        table {
            tr {
                td {
                    ul { class: "nav nav-tabs",
                        li { class: "nav-item", a { class: "nav-link active", "Table Data" } }
                        li { class: "nav-item", a { class: "nav-link", "Server stats" } }
                    }
                }
                td {
                    span { style: "padding-left: 100px", "Select Server:" }
                    select {
                        class: "form-control",
                        style: "width: 200px; display:inline; font-size:12px;",
                        onchange: move |evn| {
                            for settings in &settings.servers {
                                if settings.name == evn.data.value {
                                    global_state.write().set_active_config(settings.clone());
                                    tables_list.write().reset();
                                    right_panel.write().reset();
                                    break;
                                }
                            }
                        },
                        settings.servers.iter().map(|server|{

                            let selected = if let Some(active_config) =&active_config{
                                active_config.url == server.url
                            }else{
                                false
                            };
                            rsx!{
                                option {
                                    value: "{server.name}",
                                    selected:  selected,
                                    "{server.name}"
                                }
                            }
                        })
                    }
                }
            }
        }
    }
}

pub fn load_partitions(cx: &Scoped, table_name: String) {
    let global_state = use_shared_state::<GlobalState>(cx).unwrap();
    let tables_list_state = use_shared_state::<TablesList>(cx).unwrap();

    let right_panel_state = use_shared_state::<RightPanelState>(cx).unwrap();
    right_panel_state.write().set_loading();

    tables_list_state
        .write()
        .set_selected_table(table_name.clone());

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
    partition_key: String,
    right_panel_state: &UseSharedState<RightPanelState>,
) {
    let global_state = use_shared_state::<GlobalState>(cx).unwrap();
    let active_config = global_state.read().unwrap_active_config();
    //let right_panel_state = use_shared_state::<RightPanelState>(cx).unwrap();

    right_panel_state.write().set_loading();

    let tables_list_state = use_shared_state::<TablesList>(cx).unwrap();

    let table_name = tables_list_state
        .read()
        .get_selected_table()
        .unwrap()
        .clone();

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
 */
