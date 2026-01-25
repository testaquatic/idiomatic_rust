#![feature(coroutines, coroutine_trait, stmt_expr_attributes)]

use std::{
    f64::consts::PI,
    ops::{Coroutine, CoroutineState},
    pin::Pin,
};

fn main() {
    let mut yield_pi = #[coroutine]
    || {
        yield PI;
        "Coroutine complete!"
    };

    loop {
        match Pin::new(&mut yield_pi).resume(()) {
            CoroutineState::Yielded(val) => {
                dbg!(val);
            }
            CoroutineState::Complete(val) => {
                dbg!(val);
                break;
            }
        }
    }
}
