use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct DrawerProperties {
    pub children: Html,
    pub mobile: bool,
    pub menu_dropdown_click: Callback<MouseEvent>,
    pub dropped_down: bool
}

#[function_component(Drawer)]
pub fn drawer(props: &DrawerProperties) -> Html {
    html! {
        if props.dropped_down {
            if props.mobile {
                <aside class="mdc-drawer mdc-drawer--modal mdc-drawer--opening mdc-drawer--open">
                    {props.children.clone()}
                </aside>
                <div onclick={props.menu_dropdown_click.clone()} class="mdc-drawer-scrim mobile-only"></div>
            }
            else {
                <aside class="mdc-drawer mdc-drawer--dismissible mdc-drawer--opening mdc-drawer--open">
                    {props.children.clone()}
                </aside>
            }
        }
        else {
            <aside class="mdc-drawer mdc-drawer--dismissible mdc-drawer--closing mdc-drawer--closed">
                {props.children.clone()}
            </aside>
        }
    }
}
