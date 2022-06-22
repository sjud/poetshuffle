use std::rc::Rc;
use yew::{Reducible, UseReducerHandle};

pub type AudioContext = UseReducerHandle<AudioOptions>;
#[derive(Default, PartialEq, Clone)]
pub struct AudioOptions {
    pub(crate) is_visible: bool,
    pub(crate) is_playing: bool,
    pub(crate) src: String,
}
pub enum AudioActions {
    SwitchVisibility,
    SwitchPlaying,
    SetSrc(String),
}
impl Reducible for AudioOptions {
    type Action = AudioActions;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            AudioActions::SwitchVisibility => Rc::new(Self {
                is_visible: !self.is_visible,
                ..(*self).clone()
            }),
            AudioActions::SwitchPlaying => Rc::new(Self {
                is_playing: !self.is_playing,
                ..(*self).clone()
            }),
            AudioActions::SetSrc(src) => Rc::new(Self { src, ..*self }),
        }
    }
}
