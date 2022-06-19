use std::collections::HashMap;
use std::rc::Rc;
use uuid::Uuid;
use yew::{Html, Reducible, UseReducerHandle};
use crate::components::publish::poem_list::PoemProps;
use crate::types::auth_context::AuthContext;
use indexmap::map::IndexMap;

pub type PoemListContext = UseReducerHandle<PoemListDetails>;
#[derive(Default, PartialEq, Clone)]
pub struct PoemListDetails {
    pub map:HashMap<i32,PoemProps>,
    pub auth_ctx:AuthContext,
}

pub enum PoemListActions {
    MoveUp(i32),
    MoveDown(i32),
    Delete(i32),
    Push(PoemProps),
    Update(PoemProps),
}
impl Reducible for  PoemListDetails {
    type Action = PoemListActions;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            PoemListActions::MoveUp(idx) => {
                let map = self.map.clone();
                let prev = if idx>0 {
                    Some(idx-1)
                } else {None};
                if prev.is_some() {
                    let prev = map.get(prev)
                }
            }
            PoemListActions::MoveDown(_) => {}
            PoemListActions::Delete(_) => {}
            PoemListActions::Push(_) => {}
            PoemListActions::Update(_) => {}
        }
    }
}
