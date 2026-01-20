#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct KitchenSink;

trait FullFeatured {}

impl<T> FullFeatured for T where
    T: Clone
        + Copy
        + std::fmt::Debug
        + Default
        + PartialEq
        + Eq
        + PartialOrd
        + Ord
        + std::hash::Hash
{
}

#[derive(Debug)]
struct Container<T: FullFeatured> {
    t: T,
}

fn main() {
    let container = Container { t: KitchenSink {} };
    println!("{:?}", container);
}
