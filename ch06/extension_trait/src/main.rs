use extension_trait::{DoubleEndedIteratorExt, ReverseExt};

fn main() {
    let forward = vec![1, 2, 3];
    let reverse = forward.reversed();
    dbg!(&forward);
    dbg!(&reverse);

    let other_reversed = forward.iter().to_reversed();
    dbg!(&other_reversed);
}
