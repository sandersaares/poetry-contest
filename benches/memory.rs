use alloc_tracker::{Allocator, Session};
use criterion::{Criterion, criterion_group, criterion_main};
use poetry_contest::{
    v01_simple, v02_less_cloning, v03_borrow_document, v04_borrow_more, v05_reserve_and_reuse,
    v06_raw_contents, v07_reuse_more, v08_faster_maps,
};

#[global_allocator]
static ALLOCATOR: Allocator<std::alloc::System> = Allocator::system();

fn entrypoint(c: &mut Criterion) {
    let allocs = Session::new();

    let mut group = c.benchmark_group("memory");

    // This can be a bit slow, so let's take not too many samples.
    group.sample_size(25);

    let allocs_op = allocs.operation("v01_simple");
    group.bench_function("v01_simple", |b| {
        b.iter(|| {
            let _span = allocs_op.measure_thread();

            v01_simple::solve();
        });
    });

    let allocs_op = allocs.operation("v02_less_cloning");
    group.bench_function("v02_less_cloning", |b| {
        b.iter(|| {
            let _span = allocs_op.measure_thread();

            v02_less_cloning::solve();
        });
    });

    let allocs_op = allocs.operation("v03_borrow_document");
    group.bench_function("v03_borrow_document", |b| {
        b.iter(|| {
            let _span = allocs_op.measure_thread();

            v03_borrow_document::solve();
        });
    });

    let allocs_op = allocs.operation("v04_borrow_more");
    group.bench_function("v04_borrow_more", |b| {
        b.iter(|| {
            let _span = allocs_op.measure_thread();

            v04_borrow_more::solve();
        });
    });

    let allocs_op = allocs.operation("v05_reserve_and_reuse");
    group.bench_function("v05_reserve_and_reuse", |b| {
        b.iter(|| {
            let _span = allocs_op.measure_thread();

            v05_reserve_and_reuse::solve();
        });
    });

    let allocs_op = allocs.operation("v06_raw_contents");
    group.bench_function("v06_raw_contents", |b| {
        b.iter(|| {
            let _span = allocs_op.measure_thread();

            v06_raw_contents::solve();
        });
    });

    let allocs_op = allocs.operation("v07_reuse_more");
    group.bench_function("v07_reuse_more", |b| {
        b.iter(|| {
            let _span = allocs_op.measure_thread();

            v07_reuse_more::solve();
        });
    });

    let allocs_op = allocs.operation("v08_faster_maps");
    group.bench_function("v08_faster_maps", |b| {
        b.iter(|| {
            let _span = allocs_op.measure_thread();

            v08_faster_maps::solve();
        });
    });

    group.finish();

    allocs.print_to_stdout();
}

criterion_group!(benches, entrypoint);
criterion_main!(benches);
