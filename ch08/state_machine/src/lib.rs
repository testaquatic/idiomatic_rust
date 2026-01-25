use std::{collections::HashMap, marker::PhantomData};

use uuid::Uuid;

pub trait SessionState {}

#[derive(Debug, Default)]
pub struct Session<State: SessionState = Initial> {
    session_id: Uuid,
    props: HashMap<String, String>,
    phantom: PhantomData<State>,
}

#[derive(Debug, Default)]
pub struct Initial;
#[derive(Debug, Default)]
pub struct Anonymous;
#[derive(Debug, Default)]
pub struct Authenticated;
#[derive(Debug, Default)]
pub struct LoggedOut;

impl SessionState for Initial {}
impl SessionState for Anonymous {}
impl SessionState for Authenticated {}
impl SessionState for LoggedOut {}

#[derive(Debug)]
pub enum ResumeResult {
    Invalid,
    Anonymous(Session<Anonymous>),
    Authenticated(Session<Authenticated>),
}

impl Session<Initial> {
    pub fn new() -> Session<Anonymous> {
        Session::<Anonymous> {
            session_id: Uuid::new_v4(),
            props: HashMap::new(),
            phantom: PhantomData,
        }
    }

    pub fn resume_from(session_id: Uuid) -> ResumeResult {
        ResumeResult::Authenticated(Session::<Authenticated> {
            session_id,
            props: HashMap::new(),
            phantom: PhantomData,
        })
    }
}

impl Session<Anonymous> {
    pub fn authenticate(
        self,
        username: &str,
        password: &str,
    ) -> Result<Session<Authenticated>, Session<Anonymous>> {
        if !username.is_empty() && !password.is_empty() {
            Ok(Session::<Authenticated> {
                session_id: self.session_id,
                props: HashMap::new(),
                phantom: PhantomData,
            })
        } else {
            Err(self)
        }
    }
}

impl Session<Authenticated> {
    pub fn update_property(&mut self, key: &str, value: &str) {
        self.props
            .entry(key.to_string())
            .and_modify(|prop| *prop = value.to_string())
            .or_insert(value.to_string());
    }

    pub fn logout(self) -> Session<LoggedOut> {
        Session {
            session_id: Uuid::nil(),
            props: HashMap::new(),
            phantom: PhantomData,
        }
    }
}
