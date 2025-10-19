use yew::prelude::*;
use yew::html;
use web_sys::window;
 // this brings in , etc

 #[function_component(GoogleLogin)]
fn google_login() -> Html {
    let onclick = Callback::from(|_| {
        if let Some(win) = window() {
            win.location().set_href("https://127.0.0.1:1235/auth/google").unwrap();
        }
    });

    html! {
        <button onclick={onclick}>{ "Login with Google" }</button>
    }
}

#[function_component(Index)]
pub fn index() -> Html {
  html! {
    <>
      <h1>{"Index!"}</h1>
      <GoogleLogin />
    </>
  }
}
