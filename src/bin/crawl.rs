use walkdir::{DirEntry, WalkDir};
fn main() {
    let home = "/home/pplanel";
    let walker = WalkDir::new(home).into_iter();
    for entry in walker.filter_entry(|e| !is_hidden(e)) {
        let entry = entry.unwrap();
        println!("{:?}", tree_magic_mini::from_filepath(entry.path()));
    }
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}
