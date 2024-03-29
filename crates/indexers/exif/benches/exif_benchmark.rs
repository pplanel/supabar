use criterion::*;

use contracts::candidate::new_file_to_process;
use contracts::indexer::Indexer;
use exif_indexer::exif_indexer::ExifIndexer;
use std::path::Path;
use tokio::runtime::Runtime;

use tokio;

fn bench_indexing_exif_file(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let test_file_path = Path::new("../test_files/IMG_2551.jpeg");
    let ftp = rt.block_on(new_file_to_process(test_file_path));

    c.bench_function("indexing_exif_file", |b| {
        b.iter(|| {
            let _indexed_document = ExifIndexer.index_file(&ftp).unwrap();
        });
    });
}

criterion_group!(benches, bench_indexing_exif_file,);

criterion_main!(benches);
