use std::collections::HashMap;
use std::rc::Rc;
use uuid::Uuid;
use yew::{Html, Reducible, UseReducerHandle};
use crate::components::publish::poem_list::PoemProps;
use crate::types::auth_context::AuthContext;

pub type PoemListContext = UseReducerHandle<PoemListDetails>;
#[derive(Default, PartialEq, Clone)]
pub struct PoemListDetails {
    pub map:HashMap<i32,PoemProps>,
}

pub enum PoemListActions {
    MoveUp(i32),
    MoveDown(i32),
    Delete(i32),
    Push(PoemProps),
    Update(PoemProps),
}
impl Reducible for  PoemListDetails {
    type Action = ();

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        self
    }
}
