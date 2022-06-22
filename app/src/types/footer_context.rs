use std::rc::Rc;
use yew::{Reducible, UseReducerHandle};

pub type FooterContext = UseReducerHandle<FooterOptions>;

/// AuthToken holds onto a String which should be a JWT token. If it's not a valid JWT token, then
/// requests requiring authorization should fail.
#[derive(PartialEq, Clone)]
pub struct FooterOptions {
    pub form: FooterForm,
}
#[derive(PartialEq, Clone)]
pub enum FooterForm {
    HomePage,
    LoginPage,
}
impl Default for FooterOptions {
    fn default() -> Self {
        Self {
            form: FooterForm::HomePage,
        }
    }
}

pub enum FooterOptionsActions {
    Transform(FooterForm),
}

impl Reducible for FooterOptions {
    type Action = FooterOptionsActions;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            FooterOptionsActions::Transform(form) => Self { form }.into(),
        }
    }
}
