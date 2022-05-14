use std::path::PathBuf;

pub struct Database;
pub struct Index;
impl Index {
    pub(crate) fn create_user_index(_config: &crate::Config) -> std::path::PathBuf {
        PathBuf::new()
    }
}
