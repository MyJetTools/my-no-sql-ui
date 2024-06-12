use std::collections::BTreeMap;

use crate::TableJsonModel;

pub struct TablesList {
    pub selected_table: Option<String>,
    pub tables: Option<BTreeMap<String, TableJsonModel>>,
    pub err: Option<String>,

    pub loading: bool,
}

impl TablesList {
    pub fn new() -> Self {
        Self {
            selected_table: None,
            tables: None,
            err: None,
            loading: false,
        }
    }

    pub fn reset(&mut self) {
        self.selected_table = None;
        self.tables = None;
        self.err = None;
        self.loading = false;
    }
    pub fn set_selected_table(&mut self, table: String) {
        self.selected_table = Some(table);
    }

    pub fn get_selected_table(&self) -> Option<String> {
        self.selected_table.clone()
    }

    pub fn set_loaded_tables(&mut self, src: Vec<TableJsonModel>) {
        let mut tables = BTreeMap::new();

        for table in src {
            tables.insert(table.name.to_string(), table);
        }

        self.tables = Some(tables);

        self.loading = false;
    }

    pub fn get_tables(&self) -> Option<Vec<TableJsonModel>> {
        let result: Vec<TableJsonModel> = self
            .tables
            .as_ref()?
            .values()
            .map(|table| table.clone())
            .collect();
        Some(result)
    }

    pub fn get_err(&self) -> Option<String> {
        self.err.clone()
    }

    pub fn set_error(&mut self, err: String) {
        self.err = Some(err);
        self.loading = false;
    }
}
