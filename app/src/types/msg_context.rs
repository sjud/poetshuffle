use crate::routes::Route;
use crate::MSG_DURATION;
use std::rc::Rc;
use yew::{Reducible, UseReducerHandle};

pub type MsgContext = UseReducerHandle<UserMessageView>;

pub fn new_green_msg_with_std_duration(body: String) -> MsgActions {
    MsgActions::NewMsg(UserMessage {
        body,
        form: MsgForm::WithDuration(MSG_DURATION),
        theme: MsgTheme::Green,
    })
}
pub fn green_msg(body: String) -> MsgActions {
    MsgActions::NewMsg(UserMessage {
        body,
        form: MsgForm::Standard,
        theme: MsgTheme::Green,
    })
}
pub fn new_red_msg_with_std_duration(body: String) -> MsgActions {
    MsgActions::NewMsg(UserMessage {
        body,
        form: MsgForm::WithDuration(MSG_DURATION),
        theme: MsgTheme::Red,
    })
}

#[derive(Default, PartialEq, Clone)]
pub struct UserMessageView {
    pub msg: Option<UserMessage>,
    pub route: Route,
}
#[derive(PartialEq, Clone)]
pub struct UserMessage {
    pub body: String,
    pub form: MsgForm,
    pub theme: MsgTheme,
}
impl Default for UserMessage {
    fn default() -> Self {
        Self {
            form: MsgForm::Standard,
            body: String::default(),
            theme: MsgTheme::Green,
        }
    }
}

type Seconds = u8;
#[derive(PartialEq, Clone)]
pub enum MsgForm {
    Standard,
    WithDuration(Seconds),
}
/// Green for not errors. Red for errors.
#[derive(PartialEq, Clone)]
pub enum MsgTheme {
    Green,
    Red,
}
#[derive(PartialEq)]
pub enum MsgActions {
    NewMsg(UserMessage),
    NewRoute(Route),
    Clear,
}

impl Reducible for UserMessageView {
    type Action = MsgActions;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            // If we get a new msg update msg for current route.
            MsgActions::NewMsg(msg) => {
                tracing::debug!("{}",msg.body);
                Self {
                msg: Some(msg),
                route: self.route.clone(),
            }
            .into()},
            // Clear msg for current route.
            MsgActions::Clear => Self {
                msg: None,
                route: self.route.clone(),
            }
            .into(),
            // When we change routes we also clear the message.
            MsgActions::NewRoute(route) => Self { msg: None, route }.into(),
        }
    }
}
