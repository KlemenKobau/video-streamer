use std::env;

use poem::{listener::TcpListener, Result, Route};
use poem_openapi::{payload::Json, OpenApi, OpenApiService};

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
    let host = env::var("HOSTNAME").unwrap_or("localhost".to_owned());
    let port = env::var("PORT").unwrap_or("8080".to_owned());

    let api_service = OpenApiService::new(Api, "Video api", "1.0")
        .server(format!("http://{}:{}/api", host, port));
    let ui = api_service.swagger_ui();
    let app = Route::new().nest("/api", api_service).nest("/", ui);

    poem::Server::new(TcpListener::bind(format!("{}:{}", host, port)))
        .run(app)
        .await
}
