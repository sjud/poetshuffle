use std::collections::HashMap;
use std::rc::Rc;
use uuid::Uuid;
use wasm_bindgen_futures::spawn_local;
use yew::{Reducible, UseReducerHandle, UseStateHandle};

/*
    The job of EditPoemListContext is to have a list of sorted poems by idx
 */
pub type EditPoemListContext = UseReducerHandle<EditPoemListData>;

#[derive(PartialEq, Clone,Default)]
pub struct EditPoemListData {
    pub unsorted_poem_uuids: Vec<Uuid>,
    pub sorted_poem_uuids: Vec<(Uuid,i32)>
}


pub enum EditPoemListAction {
    PoemUuids(Vec<Uuid>),
    PushPoemUuid(Uuid),
}
impl Reducible for EditPoemListData {
    type Action = EditPoemListAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            EditPoemListAction::PoemUuids(unsorted_poem_uuids) => Rc::new( Self {
                unsorted_poem_uuids,
                sorted_poem_uuids: self.sorted_poem_uuids.clone()
            }),
            EditPoemListAction::PushPoemUuid(poem_uuid) => Rc::new(Self{
                unsorted_poem_uuids: {
                    let mut poem_uuids = self.unsorted_poem_uuids.clone();
                    poem_uuids.push(poem_uuid);
                    poem_uuids
                },
                sorted_poem_uuids: self.sorted_poem_uuids.clone(),
            })
        }
    }
}
