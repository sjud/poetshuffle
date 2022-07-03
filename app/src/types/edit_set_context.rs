use std::collections::HashMap;
use std::rc::Rc;
use uuid::Uuid;
use yew::{Reducible, UseReducerHandle, UseStateHandle};
use crate::components::publish::poem_list::PoemProps;

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
    pub(crate) poem_uuids: Vec<Uuid>,
    pub(crate) poem_idx:HashMap<Uuid,i32>,
    pub(crate) editable_set: Option<EditableSet>,
}

impl EditableSet {
    pub fn deconstruct(self) -> (Uuid, String, String) {
        (self.set_uuid, self.title, self.link)
    }
}
pub enum EditSetDataActions {
    NewEditFlag(bool),
    PoemUuids(Vec<Uuid>),
    PushPoemUuid(Uuid),
    EditableSet(Option<EditableSet>),
    InsertIdx(Uuid,i32),
    UpdateTitle(String),
    UpdateLink(String),
}
impl Reducible for EditSetData {
    type Action = EditSetDataActions;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            // TODO less clone
            EditSetDataActions::InsertIdx(uuid,idx) => Rc::new( Self {
                new_edit_flag:self.new_edit_flag,
                poem_uuids: self.poem_uuids.clone(),
                editable_set: self.editable_set.clone(),
                poem_idx: {
                    let mut map = self.poem_idx.clone();
                    map.insert(uuid,idx);
                    map
                },
            }),
            EditSetDataActions::UpdateTitle(title) => Rc::new( Self{
                new_edit_flag:self.new_edit_flag,
                poem_uuids: self.poem_uuids.clone(),
                editable_set: Some(EditableSet{
                    set_uuid: self.editable_set.clone().unwrap().set_uuid,
                    title,
                    link: self.editable_set.clone().unwrap().link,
                }),
                poem_idx: self.poem_idx.clone(),
            }),
            EditSetDataActions::UpdateLink(link) => Rc::new(Self {
                new_edit_flag:self.new_edit_flag,
                poem_uuids: self.poem_uuids.clone(),
                editable_set: Some(EditableSet{
                    set_uuid: self.editable_set.clone().unwrap().set_uuid,
                    title: self.editable_set.clone().unwrap().title,
                    link,
                }),
                poem_idx: self.poem_idx.clone(),
            }),
            EditSetDataActions::NewEditFlag(new_edit_flag) => Rc::new(Self {
                new_edit_flag,
                poem_uuids: self.poem_uuids.clone(),
                editable_set: self.editable_set.clone(),
                poem_idx: self.poem_idx.clone(),
            }),
            EditSetDataActions::PushPoemUuid(uuid) => Rc::new(Self {
                new_edit_flag: self.new_edit_flag,
                poem_uuids:{
                    let mut uuids = self.poem_uuids.clone();
                    uuids.push(uuid);
                    uuids
                },
                editable_set: self.editable_set.clone(),
                poem_idx: self.poem_idx.clone(),

            }),
            EditSetDataActions::PoemUuids(poem_uuids) => Rc::new(Self {
                new_edit_flag: self.new_edit_flag,
                poem_uuids,
                editable_set: self.editable_set.clone(),
                poem_idx: self.poem_idx.clone(),

            }),
            EditSetDataActions::EditableSet(editable_set) => Rc::new(Self {
                new_edit_flag: self.new_edit_flag,
                poem_uuids:self.poem_uuids.clone(),
                editable_set,
                poem_idx: self.poem_idx.clone(),

            }),
        }
    }
}
