fn main() {
    let left_value = || 1;
    let right_value = || 2;

    let adder = |left: fn() -> i32, right: fn() -> i32| -> i32 {
        return left() + right();
    };
    println!(
        "{} + {} = {}",
        left_value(),
        right_value(),
        adder(left_value, right_value)
    );
}
