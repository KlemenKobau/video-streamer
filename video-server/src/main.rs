use envconfig::Envconfig;
use poem::{listener::TcpListener, EndpointExt, Result, Route};
use poem_openapi::OpenApiService;

mod api;
mod config;
mod dto;
mod video;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let config = config::Config::init_from_env().unwrap();

    let api_service = OpenApiService::new(api::Api, "Video api", "1.0").server(format!(
        "http://{}:{}/api",
        config.host(),
        config.port()
    ));
    let ui = api_service.swagger_ui();
    let app = Route::new()
        .nest("/api", api_service)
        .nest("/", ui)
        .data(&config);

    poem::Server::new(TcpListener::bind(format!(
        "{}:{}",
        config.host(),
        config.port()
    )))
    .run(app)
    .await
}
