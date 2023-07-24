use super::*;

pub enum RightPanelState {
    Nothing,
    Loading,
    LoadedPartitions(LoadedPartitions),
    LoadedRows(LoadedRows),
    NoPartitions(String),
}

impl RightPanelState {
    pub fn new() -> Self {
        Self::Nothing
    }

    pub fn as_ref(&self) -> &Self {
        self
    }

    pub fn promote_to_loaded_rows(
        &mut self,
        partition_key: String,
        rows: Vec<Vec<(String, String)>>,
    ) {
        match self {
            RightPanelState::LoadedPartitions(partitions) => {
                let mut new_state = LoadedRows {
                    partition_key,
                    partitions: Vec::new(),
                    rows,
                };

                std::mem::swap(&mut new_state.partitions, &mut partitions.partitions);
                *self = RightPanelState::LoadedRows(new_state);
            }
            RightPanelState::LoadedRows(rows_model) => {
                rows_model.partition_key = partition_key;
                rows_model.rows = rows;
            }
            _ => {
                panic!("We can promote to LoadedRows from LoadedPartitions only");
            }
        }
    }
}
