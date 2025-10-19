use yew::prelude::*;
use yew_oauth2::prelude::*;
use yew_oauth2::oauth2::*;
use yew::html;

use crate::components::navbar::Navbar;

#[derive(Properties, PartialEq)]
pub struct MainLayoutProperties {
    pub children: Html
}

#[function_component(MainLayout)]
pub fn main_layout(props: &MainLayoutProperties) -> Html {
    let agent = use_auth_agent().expect("Must be nested inside an OAuth2 component");

    let login: Callback<MouseEvent> = use_callback(agent.clone(), |_, agent| {
        let _ = agent.start_login();
    });

    html!(
    <>
      <Navbar>
        {props.children.clone()}
        <br/>
        <span class="mobile-only">{"mobile-only"}</span>
        <span class="desktop-only">{"desktop-only"}</span>
      </Navbar>
      // <main class="main-content mdc-drawer-app-content" id="main-content">
      //   <div class="mdc-top-app-bar--fixed-adjust">
      //       {props.children.clone()}
      //       <br/>
      //       <span class="mobile-only">{"mobile-only"}</span>
      //       <span class="desktop-only">{"desktop-only"}</span>
      //   </div>
      // </main>
    </>
  )
}
