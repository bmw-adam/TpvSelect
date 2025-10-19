use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct DrawerContentProperties {
    pub children: Html
}

#[function_component(DrawerContent)]
pub fn drawer_content(props: &DrawerContentProperties) -> Html {
    html! {
        <div class="mdc-drawer__content">
            {props.children.clone()}
        </div>
    }
}
