use criterion::*;

use contracts::candidate::new_file_to_process;
use contracts::indexer::Indexer;
use csv_indexer::csv_indexer::CsvIndexer;
use std::path::Path;
use tokio::runtime::Runtime;

fn bench_indexing_csv_file(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let test_file_path = Path::new("../test_files/data.csv");
    let ftp = rt.block_on(new_file_to_process(test_file_path));

    c.bench_function("indexing_csv_file", |b| {
        b.iter(|| {
            let _indexed_document = CsvIndexer.index_file(&ftp).unwrap();
        });
    });
}

criterion_group!(benches, bench_indexing_csv_file,);

criterion_main!(benches);
