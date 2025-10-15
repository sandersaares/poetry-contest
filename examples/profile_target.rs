/// When profiling, it is important to always use a fixed number of
/// iterations so that the results are comparable between runs.
const ITERATIONS: usize = 25;

fn main() {
    for _ in 0..ITERATIONS {
        poetry_contest::v01_simple::solve();
    }
}
