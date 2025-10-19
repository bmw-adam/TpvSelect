use yew::prelude::*;
use yew_oauth2::oauth2::*;
use yew::html;
use yew_router::prelude::{BrowserRouter, Switch};

use crate::routes::{Route, switch};

// #[function_component(App)]
// pub fn app() -> Html {
//     let config = Config::new(
//         "147152772119-8oh8s2lgf5t2pc1g66fbgcef73hgocef.apps.googleusercontent.com",
//         "https://accounts.google.com/o/oauth2/auth",
//         "https://localhost/api/token"
//     );

//     html!(
//       <OAuth2 {config}>
//         <AppMain/>
//       </OAuth2>
//   )
// }

#[function_component(App)]
pub fn app() -> Html {
  let config = Config::new(
      "147152772119-8oh8s2lgf5t2pc1g66fbgcef73hgocef.apps.googleusercontent.com",
      "https://accounts.google.com/o/oauth2/auth",
      "https://localhost/api/token"
  );
  html! {
    <BrowserRouter>
      <OAuth2 {config}>
        <Switch<Route> render={switch} />
      </OAuth2>
    </BrowserRouter>
  }
}
