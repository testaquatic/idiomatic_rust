use pizza::Pizza;

fn main() {
    let pizza = Pizza::new(vec![
        "tomato sauce".into(),
        "mushrooms".into(),
        "mozzarella".into(),
        "pepperoni".into(),
    ]);
    println!("{:#?}", pizza);
}
