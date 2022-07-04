use std::collections::HashMap;
use std::rc::Rc;
use uuid::Uuid;
use wasm_bindgen_futures::spawn_local;
use yew::{Reducible, UseReducerHandle, UseStateHandle};


pub type EditPoemContext = UseReducerHandle<EditPoemData>;

#[derive(PartialEq, Clone,Default)]
pub struct EditPoemData {
    pub poem_uuid: Uuid,
    pub set_uuid: Uuid,
    pub banter_uuid: Option<Uuid>,
    pub title: String,
}


pub enum EditPoemAction {
    UpdateTitle(String),
}
impl Reducible for EditPoemData {
    type Action = EditPoemAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            EditPoemAction::UpdateTitle(title) => Rc::new(Self{
                poem_uuid: self.poem_uuid,
                set_uuid: self.set_uuid,
                banter_uuid: self.banter_uuid,
                title,
            })
        }
    }
}
