use core::f64;

macro_rules! special_println {
    ($($arg:tt)*) => {
        println!("Printed specially: {}", format!($($arg)*))
    };
}

macro_rules! var_print {
    ($($v:ident),*) => {
        println!(concat!($(stringify!($v),"={:?} "),*), $($v),*)
    };
}

fn main() {
    special_println!("with argument of {}", 5);
    let counter = 7;
    let gauge = f64::consts::PI;
    let name = "Peter";
    var_print!(counter, gauge, name);
}
