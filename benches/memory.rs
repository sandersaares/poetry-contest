use alloc_tracker::{Allocator, Session};
use criterion::{Criterion, criterion_group, criterion_main};
use poetry_contest::{
    v01_naive, v02_borrow_document, v03_borrow_more, v04_reserve_and_reuse, v05_raw_contents,
};

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

    let allocs_op = allocs.operation("v03_borrow_more");
    group.bench_function("v03_borrow_more", |b| {
        b.iter(|| {
            let _span = allocs_op.measure_thread();

            v03_borrow_more::solve();
        });
    });

    let allocs_op = allocs.operation("v04_reserve_and_reuse");
    group.bench_function("v04_reserve_and_reuse", |b| {
        b.iter(|| {
            let _span = allocs_op.measure_thread();

            v04_reserve_and_reuse::solve();
        });
    });

    let allocs_op = allocs.operation("v05_raw_contents");
    group.bench_function("v05_raw_contents", |b| {
        b.iter(|| {
            let _span = allocs_op.measure_thread();

            v05_raw_contents::solve();
        });
    });

    group.finish();

    allocs.print_to_stdout();
}

criterion_group!(benches, entrypoint);
criterion_main!(benches);
