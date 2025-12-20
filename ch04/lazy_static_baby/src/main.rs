use lazy_static::lazy_static;

lazy_static! {
    static ref POPULAR_BABY_NAMES_2020: Vec<String> =
        vec!["Olivia".into(), "Liam".into(), "Emma".into(), "Noah".into()];
}

fn main() {
    println!(
        "popular baby names of 2020: {:?}",
        POPULAR_BABY_NAMES_2020.as_slice()
    );
}
