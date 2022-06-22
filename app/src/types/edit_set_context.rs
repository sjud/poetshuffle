use std::collections::HashMap;
use std::rc::Rc;
use uuid::Uuid;
use yew::{Reducible, UseReducerHandle};

#[derive(Default, PartialEq, Clone)]
pub struct EditableSet {
    pub set_uuid: Uuid,
    pub collection_title: String,
    pub collection_link: String,
}

pub type EditSetContext = UseReducerHandle<EditSetData>;
#[derive(Default, PartialEq, Clone)]
pub struct EditSetData {
    pub(crate) new_edit_flag: bool,
    pub(crate) poem_list_data: PoemListData,
    pub(crate) editable_set: Option<EditableSet>,
}
#[derive(Default, PartialEq, Clone)]
pub struct PoemListData {
    pub(crate) poem_uuids: Vec<Uuid>,
    pub(crate) poem_load_flags: HashMap<Uuid, bool>,
}
impl EditableSet {
    pub fn deconstruct(self) -> (Uuid, String, String) {
        (self.set_uuid, self.collection_title, self.collection_link)
    }
}
pub enum EditSetDataActions {
    NewEditFlag(bool),
    PoemUuids(Vec<Uuid>),
    PoemLoadFlag((Uuid, bool)),
    EditableSet(Option<EditableSet>),
}
impl Reducible for EditSetData {
    type Action = EditSetDataActions;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            EditSetDataActions::NewEditFlag(new_edit_flag) => Rc::new(Self {
                new_edit_flag,
                poem_list_data: self.poem_list_data.clone(),
                editable_set: self.editable_set.clone(),
            }),
            EditSetDataActions::PoemUuids(poem_uuids) => Rc::new(Self {
                new_edit_flag: self.new_edit_flag,
                poem_list_data: {
                    PoemListData {
                        poem_uuids: poem_uuids.clone(),
                        poem_load_flags: {
                            let mut map = HashMap::new();
                            for uuid in poem_uuids {
                                map.insert(uuid, true);
                            }
                            map
                        },
                    }
                },
                editable_set: self.editable_set.clone(),
            }),
            EditSetDataActions::EditableSet(editable_set) => Rc::new(Self {
                new_edit_flag: self.new_edit_flag,
                poem_list_data: self.poem_list_data.clone(),
                editable_set,
            }),
            EditSetDataActions::PoemLoadFlag((uuid, flag)) => Rc::new(Self {
                new_edit_flag: self.new_edit_flag,
                poem_list_data: PoemListData {
                    poem_uuids: self.poem_list_data.poem_uuids.clone(),
                    poem_load_flags: {
                        let mut map = self.poem_list_data.poem_load_flags.clone();
                        map.entry(uuid).insert_entry(flag);
                        map
                    },
                },
                editable_set: self.editable_set.clone(),
            }),
        }
    }
}
