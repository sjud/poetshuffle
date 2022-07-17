use std::rc::Rc;
use yew::{Reducible, UseReducerHandle};

pub type TranscriptContext = UseReducerHandle<TranscriptOptions>;
#[derive(Default, PartialEq, Clone)]
pub struct TranscriptOptions {
    pub(crate) src: Option<String>,
    pub(crate) text: Option<String>,
    pub(crate) mount_reader:bool,
}
pub enum TranscriptActions {
    SetSrc(Option<String>),
    UpdateText(Option<String>),
    MountReader(bool),
}
impl Reducible for TranscriptOptions {
    type Action = TranscriptActions;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            TranscriptActions::SetSrc(src) => Rc::new(
                Self {
                    src,
                    text:self.text.clone(),
                    mount_reader:self.mount_reader
                }),
            TranscriptActions::UpdateText(text) => Rc::new(
                Self{
                    text,
                    src:self.src.clone(),
                    mount_reader:self.mount_reader
                }),
            TranscriptActions::MountReader(mount_reader) => Rc::new(
                Self{
                    text:self.text.clone(),
                    src:self.src.clone(),
                    mount_reader,
                }),
        }
    }
}
