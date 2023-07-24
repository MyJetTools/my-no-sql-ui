use crate::{states::*, ui_command::UiCommand};
use dioxus::prelude::*;
use elements::*;
use futures::StreamExt;
mod actions;
mod elements;
mod settings_model;
mod states;
mod ui_command;

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

    use_shared_state_provider(cx, || Tables::new());

    use_shared_state_provider(cx, || SelectedTable::new());

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
            *settings = GlobalState::Settings(result);
        });
    }

    match global_state.read().as_ref() {
        &GlobalState::ReadingSettings => render! { div { "Reading settings..." } },
        &GlobalState::Settings(_) => render! { working_app {} },
    }
}

fn working_app(cx: Scope) -> Element {
    actions::get_list_of_tables(&cx);

    let selected_table = use_shared_state::<SelectedTable>(cx).unwrap();
    let selected_table_owned = selected_table.to_owned();

    let right_panel_state = use_shared_state::<RightPanelState>(cx).unwrap();
    let right_panel_state_owned = right_panel_state.to_owned();

    let input_style = r"height: 100vh; text-align: center;";

    let main_routine = use_coroutine(cx, |mut rx: UnboundedReceiver<UiCommand>| async move {
        while let Some(msg) = rx.next().await {
            msg.handle_event(&selected_table_owned, &right_panel_state_owned)
                .await;
        }
    });

    render!(
        div { style: "{input_style}",
            table { style: "height: 100vh; width:100%;",
                tr {
                    td { style: "width: 20%; height:100vh;  vertical-align: top; text-align: left;",
                        div { style: " height: 100vh;  overflow-y: auto;",
                            left_panel {
                                on_table_selected: |selected_table| {
                                    right_panel_state.write().set_loading();
                                    main_routine.send(UiCommand::LoadPartitions(selected_table));
                                }
                            }
                        }
                    }

                    td { style: "width: 80%; height:100vh; vertical-align: top; text-align: left;",
                        div { style: " height: 100vh;  overflow-y: auto;",
                            right_part {
                                on_partition_select: |partition_key| {
                                    right_panel_state.write().set_loading();
                                    main_routine.send(UiCommand::LoadRows(partition_key));
                                }
                            }
                        }
                    }
                }
            }
        }
    )
}
