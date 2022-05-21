pub mod store;
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
    use contracts::candidate::new_file_to_process;
    use indexers::Analyzer;
    use tracing::{error, span, Level};
    use walkdir::WalkDir;

    use super::*;

    #[tokio::test]
    async fn it_works() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = tempfile::TempDir::new_in(".")?;
        let db = sled::open(temp_dir.path().join("db"))?;
        let store = pallet::Store::builder()
            .with_db(db)
            .with_index_dir(temp_dir.path())
            .finish()?;
        let analyzer = Analyzer::default();
        let walker = WalkDir::new("./fixtures").into_iter();
        let mut docs = Vec::default();
        for entry in walker.filter_entry(|e| !is_hidden(e)) {
            match entry {
                Err(_) => {
                    error!("Failed to read entry from dir walker: {:?}", entry);
                    continue;
                }
                _ => {}
            }
            let entry = entry.unwrap();
            let entry_path = entry.path().to_str().unwrap();
            let process_file_span = span!(Level::INFO, "processing_file", entry_path);
            let _process_file_entry = process_file_span.enter();
            if !entry.file_type().is_dir() {
                let entry_path = entry.path();

                match entry_path.extension() {
                    None => continue,
                    Some(extension) => {
                        if !analyzer.supported_extensions.contains(extension) {
                            continue;
                        }
                    }
                }
                let file_to_process = new_file_to_process(entry_path).await;
                let result = analyzer.analyze(file_to_process).await;
                docs.extend(result);
            }
        }
        let _ = store.create_multi(&docs);
        let docs = store.search("media_type:image_net")?;
        dbg!(docs);
        Ok(())
    }
}
