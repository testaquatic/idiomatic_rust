use builder::{Bicycle, Buildable, Builder};

fn main() {
    let bicycle = Bicycle::builder()
        .with_make("Trek")
        .with_model("Madone")
        .with_size(52)
        .with_color("purple")
        .build();

    println!("My new bike: {:#?}", bicycle);
}
