use yew::prelude::*;
use yew::html;
use web_sys::window;
 // this brings in , etc

 #[function_component(GoogleLogin)]
fn google_login() -> Html {
    let onclick = Callback::from(|_| {
        if let Some(win) = window() {
            win.location().set_href("https://78.46.206.4:1235/auth/google").unwrap();
        }
    });

    html! {
        <button onclick={onclick}>{ "Login with Google" }</button>
    }
}

#[function_component(Login)]
pub fn login() -> Html {
  html! {
    <>
      <h1>{"Login!"}</h1>
      <GoogleLogin />
    </>
  }
}
