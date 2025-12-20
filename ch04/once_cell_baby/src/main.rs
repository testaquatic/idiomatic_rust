use once_cell::sync::Lazy;

static POPULAR_BABY_NAMES_2019: Lazy<Vec<String>> =
    Lazy::new(|| vec!["Olivia".into(), "Liam".into(), "Emma".into(), "Noah".into()]);
fn main() {
    println!(
        "popular baby names of 2019: {:?}",
        POPULAR_BABY_NAMES_2019.as_slice()
    );
}
