use std::{borrow::Cow, cell::RefCell};

fn loud_moo<'a>(mut cow: Cow<'a, str>) -> Cow<'a, str> {
    if cow.contains("moo") {
        Cow::from(cow.to_mut().replace("moo", "MOO"))
    } else {
        cow
    }
}

fn main() {
    let immutable_string = String::from("This string cannot be changed");
    dbg!(&immutable_string);

    let not_so_immutable_string = RefCell::from(immutable_string);
    not_so_immutable_string
        .borrow_mut()
        .push_str("... or can it?");
    dbg!(&not_so_immutable_string);

    let cows_say_what = Cow::from("The cow goes moo");
    dbg!(&cows_say_what);
    let cows_dont_say_what = cows_say_what.clone().to_mut().replace("moo", "toot");
    dbg!(&cows_say_what);
    dbg!(&cows_dont_say_what);

    let cow_say_what = Cow::from("The cow goes moo");
    let yelling_cows = loud_moo(cow_say_what.clone());
    dbg!(&cow_say_what);
    dbg!(&yelling_cows);
}
