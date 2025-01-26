#[toml_cfg::toml_config]
pub struct Config {
    #[default("")]
    wifi_ssid: &'static str,
    #[default("")]
    wifi_psk: &'static str,
    #[default("")]
    refresh_token: &'static str,
    #[default("")]
    client_auth: &'static str,
}

fn main() {
    if !std::path::Path::new("cfg.toml").exists() {
        panic!("You need to create a `cfg.toml` file with your Wi-Fi and Spotify credentials! Please check the README.md");
    }

    embuild::espidf::sysenv::output();
}
