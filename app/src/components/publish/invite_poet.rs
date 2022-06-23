use std::sync::Arc;
use crate::services::network::GraphQlResp;
use super::*;

#[function_component(InvitePoet)]
pub fn invite_poet() -> Html {
    // We'll use these node refs in our inputs on our login form.
    let email = use_node_ref();
    // AuthContext is a ReducerHandle wrapped around Auth, so we can mutate our authtoken.
    let auth_ctx = use_context::<AuthContext>().unwrap();
    // MsgContext is used to inform user of responses.
    let msg_context = use_context::<MsgContext>().unwrap();
    let req = {
        // Clones are required because of the move in our async block.
        let email = email.clone();
        let auth = auth_ctx.clone();
        // We run this when we submit our form.
        use_async::<_, (), String>(async move {
            match auth.invite_poet(email.cast::<HtmlInputElement>().unwrap().value()).await? {
                GraphQlResp::Data(data) =>
                    msg_context.dispatch(
                        new_green_msg_with_std_duration(
                            data
                                .invite_user
                        )),
                GraphQlResp::Err(errors) =>
                    msg_context.dispatch(errors.into_msg_action()),
            }
            Ok(())
        })
    };
    // .prevent_default() is required for custom behavior for on submit buttons on forms.
    let onsubmit = Callback::from(move |e: FocusEvent| {
        e.prevent_default();
        req.run();
    });
    let form_elem = form_elem();
    let button = crate::styles::button();
    let form_css = form_css();
    html! {
        <div class={form_css.clone()}>
        <div>
            <h3>{ "Invite Poet" }</h3>
        </div>
            <form {onsubmit}>
                <input type="email" placeholder="Email" ref={email.clone()}
        class={form_elem.clone()}/>
                <br/>
                <button type="submit" disabled=false class={button.clone()}>
        { "Add Poet" } </button>
            </form>
        </div>
    }
}
