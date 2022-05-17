use walkdir::DirEntry;
/// Checks if a file or directory is hidden
pub fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use walkdir::WalkDir;

    use super::*;

    #[test]
    fn test_multi_docs_pallet_index() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = tempfile::TempDir::new_in(".")?;

        let db = sled::open(temp_dir.path().join("db"))?;

        let store = pallet::Store::builder()
            .with_db(db)
            .with_index_dir(temp_dir.path())
            .finish()?;
        let walker = WalkDir::new("../fixtures").into_iter();
        let documents = walker
            .filter_entry(|e| !is_hidden(e) && !e.file_type().is_dir())
            .filter_map(|Ok(e)| e.path().extension())
            .filter_map(|e| {})
            .collect();
        Ok(())
    }
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
