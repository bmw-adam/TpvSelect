use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AppBarContainerProperties {
    pub children: Html
}

#[function_component(AppBarContainer)]
pub fn app_bar_content(props: &AppBarContainerProperties) -> Html {
    html! {
        <div class="mdc-drawer-app-content">
            {props.children.clone()}
        </div>
    }
}
