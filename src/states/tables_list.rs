pub struct TablesList {
    pub selected_table: Option<String>,
    pub tables: Option<Vec<String>>,
}

impl TablesList {
    pub fn new() -> Self {
        Self {
            selected_table: None,
            tables: None,
        }
    }
    pub fn set_selected_table(&mut self, table: String) {
        self.selected_table = Some(table);
    }

    pub fn tables_are_loaded(&self) -> bool {
        self.tables.is_some()
    }

    pub fn get_selected_table(&self) -> Option<String> {
        self.selected_table.clone()
    }

    pub fn set_loaded_tables(&mut self, tables: Vec<String>) {
        self.tables = Some(tables);
    }

    pub fn get_tables(&self) -> Option<Vec<String>> {
        self.tables.clone()
    }

    pub fn reset(&mut self) {
        self.selected_table = None;
        self.tables = None;
    }
}
