/// When profiling, it is important to always use a fixed number of iterations to ensure
/// any data sets are comparable.
const ITERATIONS: usize = 30;

fn main() {
    for _ in 0..ITERATIONS {
        poetry_contest::v02_borrow_document::solve();
    }
}
