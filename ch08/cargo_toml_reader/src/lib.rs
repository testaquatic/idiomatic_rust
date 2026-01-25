#![feature(coroutine_trait)]
#![feature(yield_expr)]
#![feature(coroutines)]

use std::{
    fs::File,
    io::{BufRead, BufReader},
    ops::{Coroutine, CoroutineState},
    pin::Pin,
};

pub struct CargoTomlReader {
    coroutine: Pin<Box<dyn Coroutine<Yield = (usize, String), Return = ()>>>,
}
impl CargoTomlReader {
    pub fn new() -> Result<Self, std::io::Error> {
        let file = File::open("Cargo.toml")?;
        let reader = BufReader::new(file);
        let mut line = reader.lines().enumerate();

        let coroutine = Box::pin(
            #[coroutine]
            move || loop {
                match line.next() {
                    Some((line_number, Ok(line))) => {
                        yield (line_number + 1, line);
                    }
                    _ => return,
                }
            },
        );

        Ok(Self { coroutine })
    }
}

impl Iterator for CargoTomlReader {
    type Item = (usize, String);

    fn next(&mut self) -> Option<Self::Item> {
        match self.coroutine.as_mut().resume(()) {
            CoroutineState::Yielded(val) => Some(val),
            CoroutineState::Complete(_) => None,
        }
    }
}
