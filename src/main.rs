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
<script>
            
    addEventListener("resize", resize);

    function resize(){
   
        let el = document.getElementById("main");
        let winHeight = window.innerHeight;
        let winWidth = window.innerWidth;
        el.style.setProperty('--main-height', winHeight + "px");
        el.style.setProperty('--main-width', winWidth + "px");
    
        let topPanel = document.getElementById("top-panel");
        let myTopPanelHeight = topPanel.clientHeight;
        el.style.setProperty('--working-area-height', (winHeight - topPanel.clientHeight) + "px");
        el.style.setProperty('--top-panel-height', topPanel.clientHeight + "px");
    
        let leftPanel = document.getElementById("left-panel");
        el.style.setProperty('--right-panel-width', (winWidth - leftPanel.clientWidth - 10)  + "px");
    }

    setTimeout(resize, 100);
</script>

<style>
    body{
        margin: 0;
        padding: 0;
        font-family: 'Tahoma', sans-serif;
        overflow: hidden;
    }

    #top-panel{
        width:100%;
        padding:5px;
        box-shadow: 0 0 5px lightgray;
        position: absolute;
                top:0;
                left:0;
                z-index:1000;
                background-color: white;
                opacity: 0.8;
                

            }

            #main{
                overflow: hidden;
            }
            #main-wrapper{
                height: var(--main-height); 
                text-align: center;
                position:absolute;
                top:0;
                left:0;
                overflow: hidden;
            }

            #left-panel{
                width:200px;
                height:var(--main-height);
                vertical-align: top;
                text-align: left;
                overflow-y: auto;
                border-right: 1px solid lightgray;
                padding-top: var(--top-panel-height);
                margin-bottom: var(--top-panel-height);
            }

            #right-panel{
                width: var(--right-panel-width);
                height:var(--main-height);
                vertical-align: top;
                text-align: left;
                overflow-y: auto;
                padding-top: var(--top-panel-height);
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

    use_shared_state_provider(cx, || TablesList::new());

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
            });
        });
    }

    match global_state.read().as_ref() {
        GlobalState::ReadingSettings => render! { div { "Reading settings..." } },
        GlobalState::Active(_) => render! { working_app {} },
    }
}

fn working_app(cx: Scope) -> Element {
    actions::get_list_of_tables(&cx);
    let right_panel = use_shared_state::<RightPanelState>(cx).unwrap();
    render!(
        div { id: "main-wrapper",
            table { style: "height: var(--working-area-height); width:100%;",
                tr {
                    td { style: "vertical-align: top;",
                        div { id: "left-panel",
                            left_panel {
                                on_table_selected: |selected_table| {
                                    load_partitions(cx, selected_table);
                                }
                            }
                        }
                    }

                    td { style: "vertical-align: top;",
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
        div { id: "top-panel",
            span { "Select Configuration:" }
            configuration_panel {}
        }
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
        select { onchange: move |evn| {
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
