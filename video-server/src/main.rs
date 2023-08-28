use envconfig::Envconfig;
use poem::{listener::TcpListener, Result, Route};
use poem_openapi::{payload::Json, OpenApi, OpenApiService};

mod config;

struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/videos", method = "get")]
    async fn index(&self) -> Json<String> {
        Json("a".to_owned())
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let config = config::Config::init_from_env().unwrap();

    let api_service = OpenApiService::new(Api, "Video api", "1.0").server(format!(
        "http://{}:{}/api",
        config.host(),
        config.port()
    ));
    let ui = api_service.swagger_ui();
    let app = Route::new().nest("/api", api_service).nest("/", ui);

    poem::Server::new(TcpListener::bind(format!(
        "{}:{}",
        config.host(),
        config.port()
    )))
    .run(app)
    .await
}
