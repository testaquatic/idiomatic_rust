use std::sync::{Arc, Mutex};

thread_local! {
    static POPULAR_BABY_NAMES_2021: Arc<Mutex<Option<Vec<String>>>> = Arc::new(Mutex::new(None));
}

fn main() {
    let arc = POPULAR_BABY_NAMES_2021.with(|arc| arc.clone());
    let mut inner = arc.lock().expect("unable to lock mutex");

    *inner = Some(vec![
        "Olivia".into(),
        "Liam".into(),
        "Emma".into(),
        "Noah".into(),
    ]);
}
