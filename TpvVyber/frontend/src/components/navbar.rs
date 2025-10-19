use yew::prelude::*;
use yew::{function_component, html, Html, Callback};
use web_sys::window;
use material_components::components::drawer::Drawer;
use material_components::components::drawer_header::DrawerHeader;
use material_components::components::list::List;
use material_components::components::list_item::ListItem;
use material_components::components::drawer_content::DrawerContent;
use material_components::components::app_bar_container::AppBarContainer;
use material_components::components::app_bar::AppBar;
use material_components::components::app_bar_content::AppBarContent;
use material_components::icons::*;

#[derive(Properties, PartialEq)]
pub struct NavbarProps {
    pub children: Html
}

#[function_component(Navbar)]
pub fn navbar(props: &NavbarProps) -> Html {
    let menu_dropped_down = use_state(|| false);
    let is_mobile = use_state(|| false);

    // Check screen width once on mount
    {
        let is_mobile = is_mobile.clone();
        use_effect_with((), move |_| {
            if let Some(win) = window() {
                if let Ok(width) = win.inner_width() {
                    if let Some(w) = width.as_f64() {
                        is_mobile.set(w < 768.0);
                    }
                }
            }
            || ()
        });
    }


    let menu_dropdown_click = {
        let menu_dropped_down = menu_dropped_down.clone();
        Callback::from(move |_| {
            let greeting = String::from("Hi there");
            web_sys::console::log_1(&greeting.into());
            menu_dropped_down.set(!(*menu_dropped_down));
        })
    };

    html! {
        <>
        <Drawer mobile={*is_mobile} menu_dropdown_click={&menu_dropdown_click} dropped_down={*menu_dropped_down}>
            <DrawerHeader title={"Mail"} sub_title={"email@material.io"} />
            <DrawerContent>
                <List>
                    <ListItem label={"inbox"} icon={icons::HOME}></ListItem>
                    <ListItem label={"send"} icon={icons::HOME}></ListItem>
                    <ListItem label={"drafts"} icon={icons::HOME}></ListItem>
                </List>
            </DrawerContent>
        </Drawer>

        <AppBarContainer>
            <AppBar>
                <AppBarContent title="Tpv Vyber" menu_dropdown_click={&menu_dropdown_click}>
                    <span class="mdc-top-app-bar__title">{"Dismissible Drawer"}</span>
                </AppBarContent>
            </AppBar>
                <main style="margin-left: 5%;" class="main-content" id="main-content">
                    <div class="mdc-top-app-bar--fixed-adjust">
                        {props.children.clone()}
                    </div>
                </main>
        </AppBarContainer>
        </>
    }
}
