pub struct Tables {
    pub names: Option<Vec<String>>,
}

impl Tables {
    pub fn new() -> Self {
        Self { names: None }
    }
}
