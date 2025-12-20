#[derive(Debug)]
pub struct Pizza {
    topping: Vec<String>,
}

impl Pizza {
    pub fn new(topping: Vec<String>) -> Self {
        Self { topping }
    }

    pub fn topping(&self) -> &[String] {
        self.topping.as_ref()
    }

    pub fn topping_mut(&mut self) -> &mut Vec<String> {
        self.topping.as_mut()
    }

    pub fn replace_toppings(&mut self, topping: Vec<String>) -> Vec<String> {
        std::mem::replace(&mut self.topping, topping)
    }
}
