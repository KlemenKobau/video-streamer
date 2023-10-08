use app::App;

mod app;
mod config;
mod routes;

fn main() {
    yew::Renderer::<App>::new().render();
}
