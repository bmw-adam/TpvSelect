use yew::prelude::*;
use yew::html;
use yew_router::prelude::*;

use crate::layout::main_layout::MainLayout;
use crate::pages::index::Index;
use crate::pages::login::Login;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/counter")]
    Counter,
    #[at("/login")]
    Login,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: Route) -> Html {
  match route {
      Route::Home => html! { <MainLayout><Index /></MainLayout> },
      Route::Login => html! { <MainLayout><Login /></MainLayout> },
      Route::Counter => html! { <h1>{ "Counter Page" }</h1> },
      Route::NotFound => html! { <MainLayout><h1>{ "Stránka neexistuje :(" }</h1></MainLayout> },
  }
}
