use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub label: String,
    pub on_click: Callback<MouseEvent>,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    html! {
        <button class="waves-effect waves-light btn blue" onclick={props.on_click.clone()}>
            { &props.label }
        </button>
    }
}
