use std::{sync::Arc, sync::Weak};

pub trait Observer {
    type Subject;

    fn observe(&self, subject: &Self::Subject);
}

pub trait Observable {
    type Observer;

    fn update(&self);
    fn attach(&mut self, observer: Self::Observer);
    fn detach(&mut self, observer: Self::Observer);
}

pub struct Subject {
    observers: Vec<Weak<dyn Observer<Subject = Self>>>,
    state: String,
}

impl Subject {
    pub fn new(state: &str) -> Self {
        Self {
            observers: Vec::new(),
            state: state.to_string(),
        }
    }

    pub fn state(&self) -> &str {
        &self.state
    }
}

impl Observable for Subject {
    type Observer = Arc<dyn Observer<Subject = Self>>;

    fn update(&self) {
        self.observers
            .iter()
            .flat_map(|o| o.upgrade())
            .for_each(|o| o.observe(self));
    }

    fn attach(&mut self, observer: Self::Observer) {
        self.observers.push(Arc::downgrade(&observer));
    }

    fn detach(&mut self, observer: Self::Observer) {
        self.observers
            .retain(|f| !f.ptr_eq(&Arc::downgrade(&observer)));
    }
}

pub struct MyObserver {
    name: String,
}

impl MyObserver {
    pub fn new(name: &str) -> Arc<Self> {
        Arc::new(Self {
            name: name.to_string(),
        })
    }
}

impl Observer for MyObserver {
    type Subject = Subject;

    fn observe(&self, subject: &Self::Subject) {
        println!(
            "observed subject with state={:?} in {}",
            subject.state(),
            self.name
        );
    }
}
