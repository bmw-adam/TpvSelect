use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AppBarContentProperties {
    pub menu_dropdown_click: Callback<MouseEvent>,
    pub title: String,
    pub children: Html
}

#[function_component(AppBarContent)]
pub fn app_bar_content(props: &AppBarContentProperties) -> Html {
    html! {
        <div class="mdc-top-app-bar__row">
            <section class="mdc-top-app-bar__section mdc-top-app-bar__section--align-start">
                <button onclick={&props.menu_dropdown_click} class="material-icons mdc-top-app-bar__navigation-icon mdc-icon-button">{"menu"}</button>
                <span class="mdc-top-app-bar__title">{props.title.clone()}</span>
            </section>
            <section class="mdc-top-app-bar__section mdc-top-app-bar__section--align-end desktop-flex-only">
                {props.children.clone()}
            </section>
        </div>
    }
}
