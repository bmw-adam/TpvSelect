mod app;
mod pages;
mod routes;
mod layout;
mod components;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
