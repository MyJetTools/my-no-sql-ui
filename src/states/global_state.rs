use crate::settings_model::{MyNoSqlConfig, SettingsModel};

pub struct ActiveState {
    pub settings: SettingsModel,
    pub active_config: Option<MyNoSqlConfig>,
}

pub enum GlobalState {
    ReadingSettings,
    Active(ActiveState),
}

impl GlobalState {
    pub fn is_reading_settings(&self) -> bool {
        matches!(self, GlobalState::ReadingSettings)
    }

    pub fn as_ref(&self) -> &Self {
        self
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

    pub fn set_active_config(&mut self, config: MyNoSqlConfig) {
        match self {
            GlobalState::Active(model) => model.active_config = Some(config),
            _ => {
                panic!("We can get selected table only in Active state");
            }
        }
    }
}
