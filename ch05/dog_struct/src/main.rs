macro_rules! dog_struct {
    ($breed:ident) => {
        struct $breed {
            name: String,
            age: i32,
            breed: String,
        }

        impl $breed {
            fn new(name: &str, age: i32) -> Self {
                Self {
                    name: name.into(),
                    age,
                    breed: stringify!($breed).into(),
                }
            }
        }

        impl Dog for $breed {
            fn name(&self) -> &String {
                &self.name
            }

            fn age(&self) -> i32 {
                self.age
            }

            fn breed(&self) -> &String {
                &self.breed
            }
        }
    };
}

dog_struct!(Labrador);
dog_struct!(Golden);
dog_struct!(Poodle);

trait Dog {
    fn name(&self) -> &String;
    fn age(&self) -> i32;
    fn breed(&self) -> &String;
}

fn main() {
    let peter = Poodle::new("Peter", 7);
    println!(
        "{} is a {} of age {}",
        peter.name(),
        peter.breed(),
        peter.age()
    );
}
