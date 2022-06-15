use uuid::Uuid;
use yew::prelude::*;

#[derive(Properties,PartialEq)]
pub struct AcceptInvitationProps{
    pub(crate) invite_uuid:Uuid,
}
#[function_component(AcceptInvitation)]
pub fn accept_invitation(props:&AcceptInvitationProps) -> Html {
    html!{

    }
}