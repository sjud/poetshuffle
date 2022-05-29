use std::rc::Rc;
use yew::{Reducible, UseReducerHandle};

pub type AuthContext = UseReducerHandle<AuthToken>;


/// AuthToken holds onto a String which should be a JWT token. If it's not a valid JWT token, then
/// requests requiring authorization should fail.
#[derive(Default, PartialEq, Clone)]
pub struct AuthToken {
    token: String,
}

pub enum AuthTokenAction {
    Set(String)
}

impl Reducible for AuthToken {
    type Action = AuthTokenAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            AuthTokenAction::Set(token) => {
                Self { token }.into()
            }
        }
    }
}