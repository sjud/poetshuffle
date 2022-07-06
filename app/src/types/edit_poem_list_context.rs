use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use uuid::Uuid;
use wasm_bindgen_futures::spawn_local;
use yew::{Reducible, UseReducerHandle, UseStateHandle};
use crate::MSG_DURATION;
use crate::services::network::GraphQlResp;
use crate::types::auth_context::AuthToken;
use crate::types::msg_context::{MsgActions, MsgContext, MsgForm, MsgTheme, new_red_msg_with_std_duration, UserMessage};

/*
    The job of EditPoemListContext is to have a list of sorted poems by idx
 */
pub type EditPoemListContext = UseReducerHandle<EditPoemListData>;

#[derive(PartialEq, Clone,Default)]
pub struct EditPoemListData {
    pub poems: Vec<PoemData>,
}
#[derive(PartialEq, Clone)]
pub struct PoemData{
    pub(crate) uuid:Uuid,
    pub(crate) title:String,
    pub(crate) idx:i64,
}
impl EditPoemListData{
    pub fn sorted_poem_data(&self) -> Vec<PoemData> {
        let mut poem_data = self.poems.clone();
        poem_data.sort_by(|poem_a,poem_b|poem_a.idx.cmp(&poem_b.idx));
        poem_data
    }
}

pub enum EditPoemListAction {
    PushPoemData(PoemData),
}
impl Reducible for EditPoemListData {
    type Action = EditPoemListAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            EditPoemListAction::PushPoemData(data) => Rc::new(Self{
                poems: {
                    let mut poems = self.poems.clone();
                    poems.push(data);
                    poems
                },
            })
        }
    }
}
