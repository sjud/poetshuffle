use super::*;


#[function_component(DeletePoem)]
pub fn delete_poem(props:&PoemProps) -> Html {
    let edit_poem_list_ctx = use_context::<EditPoemListContext>().unwrap();
    let msg_ctx = use_context::<MsgContext>().unwrap();
    let auth_ctx = use_context::<AuthContext>().unwrap();
    let delete = {
        let auth = auth_ctx.clone();
        let poem_uuid = props.uuid;
        use_async::<_,(),String>(async move {
            match auth.delete_poem(poem_uuid).await? {
                GraphQlResp::Data(data) => {
                    edit_poem_list_ctx.dispatch(EditPoemListAction::DeletePoemData(
                        edit_poem_list_ctx.find_by_poem_uuid(poem_uuid).unwrap()
                    ));
                    msg_ctx.dispatch(
                        new_green_msg_with_std_duration(
                            data.delete_poem)
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
        delete.run()
    });
    html!{
        <div>
        <button {onclick}>{"Delete Poem"}</button>
        </div>
    }
}

#[function_component(DeleteBanter)]
pub fn delete_banter(props:&BanterProps) -> Html {
    let edit_poem_list_ctx = use_context::<EditPoemListContext>().unwrap();
    let msg_ctx = use_context::<MsgContext>().unwrap();
    let auth_ctx = use_context::<AuthContext>().unwrap();
    let delete = {
        let auth = auth_ctx.clone();
        let props = props.clone();
        use_async::<_,(),String>(async move {
            match auth.delete_banter(props.poem_props.uuid,
                                     props.banter_uuid.unwrap()).await? {
                GraphQlResp::Data(data) => {
                    edit_poem_list_ctx.dispatch(EditPoemListAction::UpdatePoemWithBanter{
                        poem_uuid: props.poem_props.uuid,
                        banter_uuid: None,
                    });
                    msg_ctx.dispatch(
                        new_green_msg_with_std_duration(data.delete_banter)
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
        delete.run()
    });
    html!{
        <div>
        <button {onclick}>{"Delete Banter"}</button>
        </div>
    }}