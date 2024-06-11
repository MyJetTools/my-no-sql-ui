use std::rc::Rc;

pub struct EnvListState {
    pub items: Option<Vec<Rc<String>>>,
    pub selected_env: Option<Rc<String>>,
    pub loading: bool,
}

impl EnvListState {
    pub fn new() -> Self {
        Self {
            items: None,
            loading: false,
            selected_env: None,
        }
    }

    pub fn set_items(&mut self, items: Vec<String>) {
        let items: Vec<Rc<String>> = items.into_iter().map(|itm| Rc::new(itm)).collect();

        if items.len() > 0 {
            self.selected_env = Some(items[0].clone());
        }
        self.items = Some(items);
        self.loading = false;
    }

    pub fn set_active_env(&mut self, selected_env: Rc<String>) {
        self.selected_env = Some(selected_env);
    }
}
