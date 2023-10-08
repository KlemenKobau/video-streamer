use yew_router::Routable;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    VideoList,
    #[at("/video/:id")]
    Video { id: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}
