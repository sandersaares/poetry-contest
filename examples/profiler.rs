/// When profiling, it is important to always use a fixed number of iterations to ensure
/// any data sets are comparable.
const ITERATIONS: usize = 100;

fn main() {
    for _ in 0..ITERATIONS {
        poetry_contest::v07_faster_maps::solve();
    }
}
