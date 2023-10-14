use common::dto::video::VideoListDto;
use gloo_net::http::Request;
use yew::{
    function_component, html, use_context, use_effect_with_deps, use_state, ContextProvider, Html,
    Properties,
};
use yew_router::{prelude::Link, BrowserRouter, Switch};

use crate::{config::Config, routes::Route};

fn main_route_switch(route: Route) -> Html {
    match route {
        Route::VideoList => html! { <VideoList></VideoList> },
        Route::Video { id } => html! { <Video video_id={id}></Video>},
        Route::NotFound => html! { {"Not found"} },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let api_base_url = option_env!("API_BASE_URL").unwrap_or("/api").to_owned();

    let cont = use_state(|| Config {
        api_base_path: api_base_url,
    });

    html! {
        <BrowserRouter>
            <ContextProvider<Config> context={(*cont).clone()}>
                <main>
                    <Switch<Route> render={main_route_switch}></Switch<Route>>
                </main>
            </ContextProvider<Config>>
            <script src="https://vjs.zencdn.net/8.5.2/video.min.js"></script>
        </BrowserRouter>
    }
}

#[function_component(VideoList)]
fn video_list() -> Html {
    let config = use_context::<Config>().unwrap();

    let videos = use_state(|| VideoListDto { video_list: vec![] });

    // TODO handle showing error in HTML
    let error_shown = use_state(|| false);
    {
        let videos = videos.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let req_result = Request::get(&format!("{}/videos", config.api_base_path))
                        .send()
                        .await;

                    match req_result {
                        Ok(c) => {
                            let fetched_videos = c.json().await;
                            match fetched_videos {
                                Ok(c) => videos.set(c),
                                Err(_) => error_shown.set(true),
                            }
                        }
                        Err(_) => error_shown.set(true),
                    }
                });
                || ()
            },
            (),
        );
    }

    html! {
        <div>
            { videos.video_list.iter().map(|x| {
                html! {
                    <Link<Route> to={ Route::Video { id: x.uuid.clone() } }>{ &x.name }</Link<Route>>
                }
            }).collect::<Html>() }
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct VideoProps {
    video_id: String,
}

#[function_component(Video)]
fn video(video_props: &VideoProps) -> Html {
    let config = use_context::<Config>().unwrap();
    let video_file_url = format!("{}/videos/{}", config.api_base_path, video_props.video_id);

    html! {
        <video-js id="hls-example" controls=true class="video-js vjs-default-skin" width="400" height="300" data-setup="'{}'">
            <source type="application/x-mpegURL" src={video_file_url}/>
        </video-js>
    }
}
