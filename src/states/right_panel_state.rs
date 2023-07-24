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

    pub fn set_loading(&mut self) {
        match self {
            Self::LoadedPartitions(model) => {
                model.loading = true;
            }
            Self::LoadedRows(model) => {
                model.loading = true;
            }
            _ => {
                *self = RightPanelState::Loading;
            }
        }
    }

    pub fn promote_to_loaded_rows(
        &mut self,
        partition_key: String,
        rows: Vec<Vec<(String, String)>>,
    ) {
        match self {
            RightPanelState::LoadedPartitions(partitions) => {
                let mut new_state = LoadedRows {
                    loading: false,
                    partition_key,
                    partitions: Vec::new(),
                    rows,
                };

                std::mem::swap(&mut new_state.partitions, &mut partitions.partitions);
                *self = RightPanelState::LoadedRows(new_state);
            }
            RightPanelState::LoadedRows(model) => {
                model.partition_key = partition_key;
                model.rows = rows;
                model.loading = false;
            }
            _ => {
                panic!("We can promote to LoadedRows from LoadedPartitions only");
            }
        }
    }

    pub fn is_loading(&self) -> bool {
        match self {
            Self::Loading => true,
            RightPanelState::LoadedPartitions(model) => model.loading,
            RightPanelState::LoadedRows(model) => model.loading,
            _ => false,
        }
    }
}
