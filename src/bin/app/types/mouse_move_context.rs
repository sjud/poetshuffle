use std::rc::Rc;
use yew::{Reducible, UseReducerHandle};

pub type MouseMoveContext = UseReducerHandle<MouseMove>;
#[derive(Default, PartialEq, Clone)]
pub struct MouseMove {
    pub(crate) did_mouse_move: bool,

}
pub enum MouseMoveActions {
    MouseMoved,
}
impl Reducible for MouseMove {
    type Action = MouseMoveActions;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            MouseMoveActions::MouseMoved => {
                Rc::new(
                    Self{
                        did_mouse_move:true
                    }
                )
            }
        }
    }
}
