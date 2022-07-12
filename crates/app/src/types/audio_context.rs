use std::rc::Rc;
use yew::{Reducible, UseReducerHandle};

pub type AudioContext = UseReducerHandle<AudioOptions>;
#[derive(Default, PartialEq, Clone)]
pub struct AudioOptions {
    pub(crate) src: Option<String>,
}
pub enum AudioActions {
    SetSrc(Option<String>),
}
impl Reducible for AudioOptions {
    type Action = AudioActions;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            AudioActions::SetSrc(src) => Rc::new(Self {
                src, ..*self }),
        }
    }
}
