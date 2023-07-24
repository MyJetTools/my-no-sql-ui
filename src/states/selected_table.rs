pub struct SelectedTable {
    name: Option<String>,
}

impl SelectedTable {
    pub fn new() -> Self {
        Self { name: None }
    }
    pub fn set_selected_table(&mut self, name: String) {
        self.name = Some(name);
    }

    pub fn get_selected_table(&self) -> Option<&String> {
        self.name.as_ref()
    }
}
