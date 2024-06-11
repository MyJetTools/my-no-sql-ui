use std::rc::Rc;

#[derive(Clone)]
pub struct LoadedPartitions {
    pub table_name: Rc<String>,
    pub partitions: Vec<Rc<String>>,
}
