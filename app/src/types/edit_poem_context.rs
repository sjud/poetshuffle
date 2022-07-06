use std::collections::HashMap;
use std::rc::Rc;
use uuid::Uuid;
use wasm_bindgen_futures::spawn_local;
use yew::{Reducible, UseReducerHandle, UseStateHandle};
use crate::components::publish::poem_list::PoemProps;


pub type EditPoemContext = UseReducerHandle<EditPoemData>;

#[derive(PartialEq, Clone)]
pub struct EditPoemData {
    pub props: PoemProps,
}


pub enum EditPoemAction {
    UpdateProps(PoemProps),
}
impl Reducible for EditPoemData {
    type Action = EditPoemAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            EditPoemAction::UpdateProps(props) =>
                Rc::new(Self{ props })
        }
    }
}
