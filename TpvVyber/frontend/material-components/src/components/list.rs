use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ListProperties {
    pub children: Html
}

#[function_component(List)]
pub fn list(props: &ListProperties) -> Html {
    html! {
        <ul class="mdc-list">
            {props.children.clone()}
        </ul>
    }
}
