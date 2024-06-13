use std::rc::Rc;

use crate::LoadedRows;

#[derive(Clone)]
pub struct RightPanelState {
    pub env: Option<Rc<String>>,
    pub table_name: Option<String>,
    pub partitions: Option<Rc<Vec<Rc<String>>>>,
    pub selected_partition: Option<Rc<String>>,
    pub loading_partitions: bool,
    pub error: Option<String>,
    pub loaded_rows: Option<Rc<LoadedRows>>,
    pub loading_rows: bool,
    pub filter_line: String,
}

impl RightPanelState {
    pub fn new() -> Self {
        Self {
            env: None,
            table_name: None,
            partitions: None,
            selected_partition: None,
            loading_partitions: false,
            error: None,
            loaded_rows: None,
            loading_rows: false,
            filter_line: "".to_string(),
        }
    }

    pub fn reset(&mut self) {
        self.env = None;
        self.table_name = None;
        self.partitions = None;
        self.selected_partition = None;
        self.loading_partitions = false;
        self.error = None;
        self.loaded_rows = None;
        self.loading_rows = false;
    }

    pub fn unwrap_table_name(&self) -> String {
        self.table_name.clone().unwrap()
    }

    pub fn unwrap_env(&self) -> Rc<String> {
        self.env.clone().unwrap()
    }

    pub fn load_partitions(&mut self, env: Rc<String>, table_name: String) {
        self.env = Some(env.clone());
        self.table_name = Some(table_name);
        self.loading_partitions = false;
        self.partitions = None;
        self.error = None;
        self.selected_partition = None;
        self.loaded_rows = None;
        self.loading_rows = false;
        self.filter_line = "".to_string();
    }

    pub fn set_loading_partitions(&mut self) {
        self.loading_partitions = true;
    }

    pub fn set_loaded_partitions(&mut self, partitions: Vec<String>) {
        self.partitions = Some(Rc::new(
            partitions.into_iter().map(|itm| itm.into()).collect(),
        ));
        self.loading_partitions = false;
    }

    pub fn get_selected_partition(&self) -> Option<Rc<String>> {
        if let Some(selected_partition) = self.selected_partition.clone() {
            return Some(selected_partition);
        }

        let partitions = self.partitions.as_ref()?;

        if partitions.len() == 1 {
            return Some(partitions[0].clone());
        }

        None
    }

    pub fn set_error(&mut self, msg: String) {
        self.error = Some(msg);
    }

    pub fn select_partition(&mut self, partition_key: Rc<String>) {
        self.selected_partition = Some(partition_key);
        self.loading_rows = false;
        self.loaded_rows = None;
    }

    pub fn set_loading_rows(&mut self) {
        self.loading_rows = true;
    }

    pub fn set_loaded_rows(&mut self, data: Vec<Vec<(String, String)>>) {
        self.loaded_rows = Some(Rc::new(LoadedRows { rows: data }));
        self.loading_rows = false;
    }
}
