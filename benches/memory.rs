use alloc_tracker::{Allocator, Session};
use criterion::{Criterion, criterion_group, criterion_main};
use poetry_contest::{v01_naive, v02_borrow_document};

#[global_allocator]
static ALLOCATOR: Allocator<std::alloc::System> = Allocator::system();

fn entrypoint(c: &mut Criterion) {
    let allocs = Session::new();

    let mut group = c.benchmark_group("memory");

    // Some of the implementations can be slow, so let's not take too many samples.
    group.sample_size(10);

    let allocs_op = allocs.operation("v01_naive");
    group.bench_function("v01_naive", |b| {
        b.iter(|| {
            let _span = allocs_op.measure_thread();

            v01_naive::solve();
        });
    });

    let allocs_op = allocs.operation("v02_borrow_document");
    group.bench_function("v02_borrow_document", |b| {
        b.iter(|| {
            let _span = allocs_op.measure_thread();

            v02_borrow_document::solve();
        });
    });

    group.finish();

    allocs.print_to_stdout();
}

criterion_group!(benches, entrypoint);
criterion_main!(benches);
