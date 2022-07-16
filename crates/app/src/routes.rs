use crate::components::{
    main_menu::MainMenu,
    publish::Publish,
};
use uuid::Uuid;
use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::validation::accept_invitation::{AcceptInvitation, AcceptInvitationProps};
use crate::components::validation::admin::Admin;
use crate::components::validation::login_register::LoginRegister;
use crate::components::validation::validate_registration::ValidateRegistration;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/accept_invitation/:invite_uuid")]
    AcceptInvitation { invite_uuid: Uuid },
    #[at("/admin")]
    Admin,
    #[at("/validate_registration/:email/:code")]
    ValidateRegistration { email: String, code: String },
    #[at("/poetshuffle")]
    PoetShuffle,
    #[at("/about")]
    About,
    #[at("/login_register")]
    LoginRegister,
    #[at("/publish")]
    Publish,
    #[at("/")]
    MainMenu,
    #[not_found]
    #[at("/404")]
    NotFound,
}
pub(crate) fn switch(routes: &Route) -> Html {
    match routes {
        Route::PoetShuffle => html! {{"PoetShuffle"}},
        Route::About => html! {{"About"}},
        Route::LoginRegister => html! {<LoginRegister/>},
        Route::Publish => html! {<Publish/>},
        Route::MainMenu => html! {<MainMenu />},
        Route::NotFound => html! { {"404"}},
        Route::ValidateRegistration { email, code } => {
            let props = crate::components::validation::validate_registration::ValidateRegistrationProps {
                email: email.clone(),
                code: code.clone(),
            };
            return html! {
            <ValidateRegistration ..props/>
            };
        }
        Route::Admin => html! {<Admin/>},
        Route::AcceptInvitation { invite_uuid } => {
            let props = AcceptInvitationProps {
                invite_uuid: *invite_uuid,
            };
            return html! {
                    <AcceptInvitation ..props/>
            };
        }
    }
}
