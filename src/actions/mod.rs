mod get_list_of_tables;
pub use get_list_of_tables::*;
mod get_list_of_partition;
pub use get_list_of_partition::*;
mod get_list_of_rows;

pub const BASE_URL: &str = "http://127.0.0.1:5123";
pub use get_list_of_rows::*;
