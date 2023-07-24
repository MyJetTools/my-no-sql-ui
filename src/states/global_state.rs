use crate::settings_model::SettingsModel;

pub enum GlobalState {
    ReadingSettings,
    Settings(SettingsModel),
}

impl GlobalState {
    pub fn is_reading_settings(&self) -> bool {
        matches!(self, GlobalState::ReadingSettings)
    }

    pub fn as_ref(&self) -> &Self {
        self
    }
}
