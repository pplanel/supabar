use criterion::*;

use contracts::candidate::new_file_to_process;
use contracts::indexer::Indexer;
use pdf_indexer::pdf_indexer::PdfIndexer;
use std::path::Path;
use tokio::runtime::Runtime;

fn bench_indexing_pdf_file(c: &mut Criterion) {
    let mut group = c.benchmark_group("pdf_indexer");
    let rt = Runtime::new().unwrap();
    let test_file_path = Path::new("../test_files/Cats.pdf");
    let ftp = rt.block_on(new_file_to_process(test_file_path));
    group.bench_function("indexing_pdf_file", |b| {
        b.iter(|| {
            let _indexed_document = PdfIndexer.index_file(&ftp).unwrap();
        });
    });
    group.finish();
}

criterion_group!(benches, bench_indexing_pdf_file,);

criterion_main!(benches);
