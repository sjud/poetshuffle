
use yew::prelude::*;

#[function_component(TextReader)]
pub fn reader() -> Html{
    html!{{"Todo reader.."}}
}

#[function_component(ReadButton)]
pub fn read_button() -> Html {
    html!{
        <button>{"Read"}</button>
    }
}