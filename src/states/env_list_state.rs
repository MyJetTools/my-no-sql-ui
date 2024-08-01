use std::rc::Rc;

pub struct EnvListState {
    items: Option<Vec<Rc<String>>>,
    selected_env: Option<Rc<String>>,
}

impl EnvListState {
    pub fn new() -> Self {
        Self {
            items: None,
            selected_env: None,
        }
    }

    pub fn unwrap_envs(&self) -> Vec<Rc<String>> {
        let result = self.items.clone();

        if result.is_none() {
            panic!("No envs loaded");
        }

        result.unwrap()
    }

    pub fn has_envs(&self) -> bool {
        self.items.is_some()
    }
    pub fn get_selected_env(&self) -> Option<Rc<String>> {
        self.selected_env.clone()
    }

    pub fn set_items(&mut self, items: Vec<String>) {
        let items: Vec<Rc<String>> = items.into_iter().map(|itm| Rc::new(itm)).collect();
        self.items = Some(items);
    }

    pub fn set_active_env(&mut self, selected_env: String) {
        if self.items.is_none() {
            panic!("Should net set active env before evns are loaded");
        }

        let index = self
            .items
            .as_ref()
            .unwrap()
            .iter()
            .position(|itm| itm.as_str() == selected_env.as_str());

        match index {
            Some(index) => {
                self.selected_env = Some(self.items.as_ref().unwrap()[index].clone());
            }
            None => {
                self.selected_env = self.items.as_ref().unwrap().first().cloned();
            }
        }
    }
}
