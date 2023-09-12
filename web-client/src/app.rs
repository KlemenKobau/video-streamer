use common::dto::video::VideoListDto;
use gloo_net::http::Request;
use std::rc::Rc;
use yew::{
    function_component, html, use_context, use_effect_with_deps, use_memo, use_state,
    ContextProvider, Html,
};

use crate::config::Config;

#[function_component(App)]
pub fn app() -> Html {
    let api_base_url = option_env!("API_BASE_URL")
        .unwrap_or("http://localhost:8081")
        .to_owned();

    let cont = use_memo(
        |_| Config {
            api_base_path: api_base_url,
        },
        (),
    );

    html! {
        <ContextProvider<Rc<Config>> context={cont}>
            <main>
                <VideoList></VideoList>
            </main>
        </ContextProvider<Rc<Config>>>
    }
}

#[function_component(VideoList)]
pub fn video_list() -> Html {
    let config = use_context::<Config>().unwrap();

    let videos = use_state(|| VideoListDto { video_list: vec![] });
    {
        let videos = videos.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_videos: VideoListDto =
                        Request::get(&format!("{}/videos", config.api_base_path))
                            .send()
                            .await
                            .unwrap()
                            .json()
                            .await
                            .unwrap();
                    videos.set(fetched_videos);
                });
                || ()
            },
            (),
        );
    }

    html! {
        <> { "HELLO" } </>
    }
}
