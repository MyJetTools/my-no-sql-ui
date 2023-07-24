use dioxus::prelude::UseSharedState;

use crate::{actions, states::*};

#[derive(Debug)]
pub enum UiCommand {
    LoadPartitions(String),
    LoadRows(String),
}

impl UiCommand {
    pub async fn handle_event(
        self,
        selected_table: &UseSharedState<SelectedTable>,
        right_panel_state: &UseSharedState<RightPanelState>,
    ) {
        match self {
            UiCommand::LoadPartitions(table_name) => {
                selected_table
                    .write()
                    .set_selected_table(table_name.clone());
                let mut result = actions::get_list_of_partitions(table_name.as_str()).await;

                if result.data.len() > 1 {
                    let mut right_panel_state = right_panel_state.write();
                    *right_panel_state = RightPanelState::LoadedPartitions(LoadedPartitions {
                        table_name: table_name,
                        partitions: result.data,
                        amount: result.amount,
                    });
                    return;
                }

                if result.data.len() == 1 {
                    let partition_key = result.data.remove(0);
                    let rows = actions::get_list_of_rows(&table_name, &partition_key).await;
                    let mut right_panel_state = right_panel_state.write();
                    *right_panel_state = RightPanelState::LoadedRows(LoadedRows {
                        partition_key: partition_key.to_string(),
                        partitions: vec![partition_key],
                        rows,
                    });
                    return;
                }
                let mut right_panel_state = right_panel_state.write();
                *right_panel_state = RightPanelState::NoPartitions(table_name);
            }

            UiCommand::LoadRows(partition_key) => {
                let selected_table = {
                    let selected_table = selected_table.read();
                    selected_table.get_selected_table().unwrap().clone()
                };
                let rows = actions::get_list_of_rows(&selected_table, &partition_key).await;

                let mut right_panel_state = right_panel_state.write();
                right_panel_state.promote_to_loaded_rows(partition_key, rows);
            }
        }
    }
}
