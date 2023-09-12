use app::App;

mod app;
mod config;

fn main() {
    yew::Renderer::<App>::new().render();
}
