use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct DrawerHeaderProperties {
    pub title: String,
    pub sub_title: String
}

#[function_component(DrawerHeader)]
pub fn drawer(props: &DrawerHeaderProperties) -> Html {
    html! {
            <div class="mdc-drawer__header">
                <h3 class="mdc-drawer__title">{&props.title}</h3>
                <h6 class="mdc-drawer__subtitle">{&props.sub_title}</h6>
                <div style={"height: 5px"}></div>
                <md-divider></md-divider>
            </div>
    }
}
