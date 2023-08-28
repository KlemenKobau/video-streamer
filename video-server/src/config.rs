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
}
