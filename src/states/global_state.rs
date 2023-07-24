use crate::settings_model::{MyNoSqlConfig, SettingsModel};

pub struct ActiveState {
    pub settings: SettingsModel,
    pub active_config: Option<MyNoSqlConfig>,
    pub selected_table: Option<String>,
    pub tables: Option<Vec<String>>,
}

pub enum GlobalState {
    ReadingSettings,
    Active(ActiveState),
    Error(String),
}

impl GlobalState {
    pub fn is_reading_settings(&self) -> bool {
        matches!(self, GlobalState::ReadingSettings)
    }

    pub fn as_ref(&self) -> &Self {
        self
    }

    pub fn set_selected_table(&mut self, table: String) {
        match self {
            GlobalState::Active(model) => {
                model.selected_table = Some(table);
            }
            _ => {
                panic!("We can set selected table only in Active state");
            }
        }
    }

    pub fn get_selected_table(&self) -> Option<String> {
        match self {
            GlobalState::Active(model) => model.selected_table.clone(),
            _ => {
                panic!("We can get selected table only in Active state");
            }
        }
    }

    pub fn tables_are_loaded(&self) -> bool {
        match self {
            GlobalState::Active(model) => model.tables.is_some(),
            _ => {
                panic!("We can get selected table only in Active state");
            }
        }
    }

    pub fn set_loaded_tables(&mut self, tables: Vec<String>) {
        match self {
            GlobalState::Active(model) => {
                model.tables = Some(tables);
            }
            _ => {
                panic!("We can get selected table only in Active state");
            }
        }
    }

    pub fn set_error(&mut self, error: String) {
        *self = GlobalState::Error(error);
    }

    pub fn get_tables(&self) -> Option<Vec<String>> {
        match self {
            GlobalState::Active(model) => model.tables.clone(),
            _ => {
                panic!("We can get selected table only in Active state");
            }
        }
    }

    pub fn unwrap_as_active(&self) -> &ActiveState {
        match self {
            GlobalState::Active(model) => model,
            _ => {
                panic!("We can get selected table only in Active state");
            }
        }
    }

    pub fn unwrap_active_config(&self) -> MyNoSqlConfig {
        match self {
            GlobalState::Active(model) => model.active_config.clone().unwrap(),
            _ => {
                panic!("We can get selected table only in Active state");
            }
        }
    }
}
