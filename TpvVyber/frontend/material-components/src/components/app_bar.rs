use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AppbarProperties {
    pub children: Html
}

#[function_component(AppBar)]
pub fn app_bar(props: &AppbarProperties) -> Html {
    html! {
        <header class="mdc-top-app-bar app-bar" id="app-bar">
            { props.children.clone() }
        </header>
    }
}
