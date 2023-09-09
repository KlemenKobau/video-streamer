use envconfig::Envconfig;
use getset::Getters;

#[derive(Debug, Envconfig, Getters)]
pub struct Config {
    #[getset(get = "pub")]
    #[envconfig(from = "HOSTNAME", default = "localhost")]
    host: String,

    #[getset(get = "pub")]
    #[envconfig(from = "PORT", default = "8080")]
    port: String,

    #[getset(get = "pub")]
    #[envconfig(from = "video", nested = true)]
    video_config: VideoConfig,

    #[getset(get = "pub")]
    #[envconfig(from = "encode", nested = true)]
    encode_config: VideoEncodeConfig,
}

#[derive(Debug, Envconfig, Getters)]
pub struct VideoConfig {
    #[getset(get = "pub")]
    #[envconfig(from = "VIDEO_FOLDER_PATH", default = "videos")]
    video_folder_path: String,

    #[getset(get = "pub")]
    #[envconfig(from = "VIDEOS_FILE", default = "videos.txt")]
    videos_file: String,
}

#[derive(Debug, Envconfig, Getters)]
pub struct VideoEncodeConfig {
    #[getset(get = "pub")]
    #[envconfig(from = "TEMP_VIDEO_FOLDER", default = "videos")]
    temp_video_folder: String,
}
