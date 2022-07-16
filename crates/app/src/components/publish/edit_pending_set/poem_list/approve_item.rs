use super::*;

#[function_component(ApprovePoem)]
pub fn approve_poem(props:&PoemProps) -> Html {
    let msg_ctx = use_context::<MsgContext>().unwrap();
    let auth_ctx = use_context::<AuthContext>().unwrap();
    let poem_ctx = use_context::<EditPoemListContext>().unwrap();
    let set_uuid = use_context::<EditSetContext>()
        .unwrap()
        .editable_set
        .as_ref()
        .unwrap()
        .set_uuid;
    let approve_state = use_state(||true);
    if use_is_first_mount() {
        // Set state to opposite of data so when we update to state we switch data.
        approve_state.set(
            !poem_ctx.find_by_poem_uuid(props.uuid).unwrap().approved
        );
    }
    let approve = {
        let auth = auth_ctx.clone();
        let poem_uuid = props.uuid;
        let approve_state = approve_state.clone();
        let poem_ctx = poem_ctx.clone();
        use_async::<_,(),String>(async move {
            match auth.set_approve_poem(
                poem_uuid,set_uuid,*approve_state

            ).await? {
                GraphQlResp::Data(data) => {
                    let poem_data = poem_ctx.find_by_poem_uuid(poem_uuid).unwrap();
                    poem_ctx.dispatch(EditPoemListAction::UpdatePoemData(PoemData{
                        approved:*approve_state,..poem_data
                    }));
                    approve_state.set(!*approve_state);
                    msg_ctx.dispatch(
                        new_green_msg_with_std_duration(data.set_approve_poem)
                    );
                },
                GraphQlResp::Err(errors) => {
                    msg_ctx.dispatch(errors.into_msg_action());
                }
            }
            Ok(())
        })
    };
    let onclick= Callback::from(move|_|{
        approve.run()
    });
    let msg = {
        if *approve_state {
            "Approve Poem"
        } else {
            "Un-Approve Poem"
        }
    };
    html!{
        <button {onclick}>{msg}</button>
    }
}

#[function_component(ApproveBanter)]
pub fn approve_banter(props:&BanterProps) -> Html {
    //TODO include un-approve option
    let msg_ctx = use_context::<MsgContext>().unwrap();
    let auth_ctx = use_context::<AuthContext>().unwrap();
    let set_uuid = use_context::<EditSetContext>().unwrap()
        .editable_set
        .as_ref()
        .unwrap()
        .set_uuid;
    let approve = {
        let auth = auth_ctx.clone();
        let banter_uuid = props.banter_uuid.unwrap();
        use_async::<_,(),String>(async move {
            match auth.set_approve_banter(
                banter_uuid,set_uuid,true
            ).await? {
                GraphQlResp::Data(data) => {
                    msg_ctx.dispatch(
                        new_green_msg_with_std_duration(data.set_approve_banter)
                    );
                },
                GraphQlResp::Err(errors) => {
                    msg_ctx.dispatch(errors.into_msg_action());
                }
            }
            Ok(())
        })
    };
    let onclick= Callback::from(move|_|{
        approve.run()
    });
    html!{
        <button {onclick}>{"Approve Banter"}</button>
    }
}