use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::rc::Rc;
use uuid::Uuid;
use yew::{Reducible, UseReducerHandle};

pub type AuthContext = UseReducerHandle<AuthToken>;

/// This is copied from entities... make sure it is up to date.
/// I'd prefer one source of truth, but there are wasm target
/// build conflicts when bringing in the mess dependencies.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum UserRole {
    Admin,
    Listener,
    Moderator,
    Poet,
    SuperAdmin,
}

impl UserRole {
    pub fn from_str(role: &'static str) -> Option<Self> {
        match role {
            "Admin" => Some(Self::Admin),
            "Listener" => Some(Self::Listener),
            "Moderator" => Some(Self::Moderator),
            "Poet" => Some(Self::Poet),
            "SuperAdmin" => Some(Self::SuperAdmin),
            _ => None,
        }
    }
}

impl PartialOrd for UserRole {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let self_val = match self {
            UserRole::Listener => 0,
            UserRole::Poet => 1,
            UserRole::Moderator => 2,
            UserRole::Admin => 3,
            UserRole::SuperAdmin => 4,
        };
        let other_val = match other {
            UserRole::Listener => 0,
            UserRole::Poet => 1,
            UserRole::Moderator => 2,
            UserRole::Admin => 3,
            UserRole::SuperAdmin => 4,
        };
        if self_val < other_val {
            Some(Ordering::Less)
        } else if self_val > other_val {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Equal)
        }
    }
}

/// This is copied from entities... make sure it is up to date.
/// I'd prefer one source of truth, but there are wasm target
/// build conflicts when bringing in the mess dependencies.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Permissions {
    pub user_uuid: Uuid,
    pub user_role: UserRole,
}

/// AuthToken holds onto a String which should be a JWT token. If it's not a valid JWT token, then
/// requests requiring authorization should fail.
#[derive(PartialEq, Clone)]
pub struct AuthToken {
    pub(crate) token: Option<String>,
    pub user_uuid: Option<Uuid>,
    pub user_role: UserRole,
}
impl Default for AuthToken {
    fn default() -> Self {
        Self {
            token: None,
            user_uuid: None,
            user_role: UserRole::Listener,
        }
    }
}

pub enum AuthTokenAction {
    Set(Option<String>),
}

impl Reducible for AuthToken {
    type Action = AuthTokenAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            AuthTokenAction::Set(token) => {
                if let Some(token) = token {
                    let payload = token.split(".").collect::<Vec<&str>>()[1];
                    let payload = base64::decode(payload).unwrap();
                    if let Value::Object(map) = serde_json::from_slice(&payload).unwrap() {
                        let perm: Permissions =
                            serde_json::from_value(map.get("sub").unwrap().clone()).unwrap();
                        gloo::console::log!(&format!(
                            "Uuid: {}\n Role: {:?}",
                            perm.user_uuid, perm.user_role
                        ));
                        return Self {
                            token: Some(token),
                            user_uuid: Some(perm.user_uuid),
                            user_role: perm.user_role,
                        }
                        .into();
                    }
                }
                Self::default().into()
            }
        }
    }
}
