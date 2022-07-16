use std::rc::Rc;
use yew::{Reducible, UseReducerHandle};

pub type TranscriptContext = UseReducerHandle<TranscriptOptions>;
#[derive(Default, PartialEq, Clone)]
pub struct TranscriptOptions {
    pub(crate) src: Option<String>,
}
pub enum TranscriptActions {
    SetSrc(Option<String>),
}
impl Reducible for TranscriptOptions {
    type Action = TranscriptActions;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            TranscriptActions::SetSrc(src) => Rc::new(Self {
                src, ..*self }),
        }
    }
}
