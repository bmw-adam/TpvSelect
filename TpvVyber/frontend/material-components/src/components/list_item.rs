use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ListItemProperties {
    pub label: String,
    #[prop_or_default] // This will provide a default value if not specified
    pub icon: Option<String>, // Make icon an Option<String>
}

#[function_component(ListItem)]
pub fn list_item(props: &ListItemProperties) -> Html {
    html! {
        <li class="mdc-list-item listelelement" tabindex="0">
            <span class="mdc-list-item__ripple"></span>
            // Render the icon if it exists
            { if let Some(icon) = &props.icon {
                Html::from_html_unchecked(icon.clone().into())
            } else {
                html! {} // Render nothing if no icon is provided
            }}
            <span class="mdc-list-item__text">{ &props.label }</span>
        </li>
    }
}
