pub struct LoadedPartitions {
    pub loading: bool,
    pub table_name: String,
    pub partitions: Vec<String>,
    pub amount: usize,
}
