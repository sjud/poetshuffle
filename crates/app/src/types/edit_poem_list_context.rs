use std::rc::Rc;
use uuid::Uuid;
use yew::{Reducible, UseReducerHandle};

/*
    The job of EditPoemListContext is to have a list of sorted poems by idx
 */
pub type EditPoemListContext = UseReducerHandle<EditPoemListData>;

#[derive(PartialEq, Clone,Default)]
pub struct EditPoemListData {
    pub poems: Vec<PoemData>,
}
#[derive(PartialEq, Clone,Debug)]
pub struct PoemData{
    pub(crate) uuid:Uuid,
    pub(crate) title:String,
    pub(crate) set_uuid:Uuid,
    pub(crate) banter_uuid:Option<Uuid>,
    pub(crate) idx:i64,
}

impl EditPoemListData{
    pub fn sorted_poem_data(&self) -> Vec<PoemData> {
        let mut poem_data = self.poems.clone();
        poem_data.sort_by(|poem_a,poem_b|poem_a.idx.cmp(&poem_b.idx));
        poem_data
    }
    pub fn new_from_unsorted_poem_data(poems:Vec<PoemData>) -> Self {
        let mut poems = poems;
        poems.sort_by(|poem_a,poem_b|poem_a.idx.cmp(&poem_b.idx));
        Self{
            poems
        }
    }
    pub fn find_by_poem_uuid(&self,poem_uuid:Uuid) -> Option<PoemData> {
        self.poems
            .iter()
            .find(|&poem| poem.uuid == poem_uuid)
            .map(|poem|poem.clone())
    }
    pub fn swap_idx(&self, idx_a:i64,idx_b:i64) -> Option<Vec<PoemData>> {
        let mut poems = self.poems.clone();
        if let Some(poem_a) = poems.clone()
            .into_iter()
            .find(|poem|poem.idx==idx_a) {
            if let Some(poem_b) = poems.clone()
                .into_iter()
                .find(|poem|poem.idx==idx_b) {
                let mut poems = poems.into_iter()
                    .filter(|poem|
                        if poem.uuid == poem_b.uuid || poem.uuid == poem_a.uuid {
                            false
                        } else {
                            true
                        })
                    .collect::<Vec<PoemData>>();
                poems.push(PoemData{
                    idx:idx_a,
                    ..poem_b.clone()
                });
                poems.push(PoemData{
                    idx:idx_b,
                    ..poem_a.clone()
                });
                Some(poems)
            } else {
                None
            }
        } else {
            None
        }
    }
}

pub enum EditPoemListAction {
    PushPoemData(PoemData),
    SwapIdx(i64,i64),
    UpdatePoemData(PoemData),
    DeletePoemData(PoemData),
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
            }),
            EditPoemListAction::SwapIdx(idx_a, idx_b) => Rc::new(
                EditPoemListData::new_from_unsorted_poem_data(
                    self.swap_idx(idx_a,idx_b).unwrap()
                )
            ),
            EditPoemListAction::UpdatePoemData(poem_data) => Rc::new(Self{
                poems: vec![
                    self.poems
                        .clone()
                        .into_iter()
                        .filter(|poem|poem.uuid!=poem_data.uuid)
                        .collect::<Vec<PoemData>>(),
                    vec![poem_data]]
                    .into_iter()
                    .flatten()
                    .collect()
            }),
            EditPoemListAction::DeletePoemData(data) => Rc::new(Self{
                poems: self.poems
                    .clone()
                    .into_iter()
                    .filter(|poem|poem.uuid!=data.uuid)
                    .map(|poem|{
                        let mut poem = poem;
                        if poem.idx > data.idx{
                            poem.idx = poem.idx-1;
                        }
                        poem})
                    .collect::<Vec<PoemData>>()
            }),
        }
    }
}
