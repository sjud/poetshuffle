use std::collections::HashMap;
use std::rc::Rc;
use uuid::Uuid;
use wasm_bindgen_futures::spawn_local;
use yew::{Reducible, UseReducerHandle, UseStateHandle};

#[derive(Default, PartialEq, Clone)]
pub struct EditableSet {
    pub set_uuid: Uuid,
    pub title: String,
    pub link: String,
}

pub type EditSetContext = UseReducerHandle<EditSetData>;

#[derive(PartialEq, Clone,Default)]
pub struct EditSetData {
    pub(crate) new_edit_flag: bool,
    pub(crate) editable_set: Option<EditableSet>,
}

impl EditableSet {
    pub fn deconstruct(self) -> (Uuid, String, String) {
        (self.set_uuid, self.title, self.link)
    }
}
pub enum EditSetActions {
    NewEditFlag(bool),
    EditableSet(Option<EditableSet>),
    UpdateTitle(String),
    UpdateLink(String),
}
impl Reducible for EditSetData {
    type Action = EditSetActions;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            // TODO less clone
            EditSetActions::UpdateTitle(title) => Rc::new( Self{
                new_edit_flag:self.new_edit_flag,
                editable_set: Some(EditableSet{
                    set_uuid: self.editable_set.clone().unwrap().set_uuid,
                    title,
                    link: self.editable_set.clone().unwrap().link,
                }),
            }),
            EditSetActions::UpdateLink(link) => Rc::new(Self {
                new_edit_flag:self.new_edit_flag,
                editable_set: Some(EditableSet{
                    set_uuid: self.editable_set.clone().unwrap().set_uuid,
                    title: self.editable_set.clone().unwrap().title,
                    link,
                }),
            }),
            EditSetActions::NewEditFlag(new_edit_flag) => Rc::new(Self {
                new_edit_flag,
                editable_set: self.editable_set.clone(),

            }),
            EditSetActions::EditableSet(editable_set) => Rc::new(Self {
                new_edit_flag: self.new_edit_flag,
                editable_set,
            }),
        }
    }
}
