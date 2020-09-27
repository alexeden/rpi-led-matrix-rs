mod example_utils;
use example_utils::{create_matrix, wait};
fn main() {
    let mat = create_matrix();

    wait(1000);
}
