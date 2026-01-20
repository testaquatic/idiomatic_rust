use std::marker::PhantomData;

struct LightBulb<State: BulbState> {
    phatom: PhantomData<State>,
}

impl LightBulb<On> {
    fn turn_off(self) -> LightBulb<Off> {
        LightBulb::<Off>::default()
    }

    fn state(&self) -> &str {
        "on"
    }
}

impl LightBulb<Off> {
    fn turn_on(self) -> LightBulb<On> {
        LightBulb::<On>::default()
    }

    fn state(&self) -> &str {
        "off"
    }
}

impl<State: BulbState> Default for LightBulb<State> {
    fn default() -> Self {
        LightBulb {
            phatom: PhantomData::<State>::default(),
        }
    }
}

trait BulbState {}

struct On;

struct Off;

macro_rules! impl_bulbstate {
    ($($bulb_state:ident),*) => {
        $(impl BulbState for $bulb_state {})*
    };
}

impl_bulbstate!(On, Off);

fn main() {
    let lightbulb = LightBulb::<Off>::default();
    println!("Bulb is {}", lightbulb.state());
    let lightbulb = lightbulb.turn_on();
    println!("Bulb is {}", lightbulb.state());
    let lightbulb = lightbulb.turn_off();
    println!("Bulb is {}", lightbulb.state());
}
