use super::*;
use crate::queries::{PoemQuery,poem_query};

#[derive(Properties,PartialEq)]
pub struct PoemListProps{
    poem_uuids:Vec<Uuid>,
}
#[function_component(PoemList)]
pub fn poem_list(props:&PoemListProps) -> Html {

    html!{}
}
#[derive(Properties,PartialEq,Clone)]
pub struct PoemProps {
    pub title:String,
    pub uuid:Uuid,
    pub idx:i32,
}
#[function_component(PoemCard)]
pub fn poem_card(props: &PoemProps) -> Html {
    let auth_ctx = use_context::<AuthContext>().unwrap();
    let poem_title = use_state(||String::new());
    let poem_idx = use_state(||0);

    html!{

    }
}