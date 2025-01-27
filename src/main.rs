mod button;
mod request;
mod spotify;
mod util;
mod wifi;

use anyhow::Result;
use core::str;
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    hal::{delay::Delay, gpio::PinDriver, prelude::Peripherals},
};

use button::Button;
use spotify::{update_player, PlayerAction, SpotifyToken};
use wifi::wifi;

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

fn main() -> Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let sysloop = EspSystemEventLoop::take()?;

    // The constant `CONFIG` is auto-generated by `toml_config`.
    let app_config = CONFIG;

    let mut play_button = Button::new(peripherals.pins.gpio18);
    let mut pause_button = Button::new(peripherals.pins.gpio5);
    let mut skip_button = Button::new(peripherals.pins.gpio19);
    let mut prev_button = Button::new(peripherals.pins.gpio17);

    let mut wifi_led = PinDriver::output(peripherals.pins.gpio16)?;

    wifi_led.set_low()?;

    // Connect to the Wi-Fi network
    let _wifi = match wifi(
        app_config.wifi_ssid,
        app_config.wifi_psk,
        peripherals.modem,
        sysloop,
    ) {
        Ok(wifi) => {
            wifi_led.set_high()?;
            wifi
        }
        Err(e) => return Err(e),
    };

    let token = SpotifyToken::new(app_config.refresh_token, app_config.client_auth);

    let delay = Delay::new_default();

    loop {
        play_button.update();
        pause_button.update();
        skip_button.update();
        prev_button.update();

        if play_button.clicked() {
            update_player(PlayerAction::Play, &token);
        }
        if pause_button.clicked() {
            update_player(PlayerAction::Pause, &token);
        }
        if skip_button.clicked() {
            update_player(PlayerAction::Skip, &token);
        }
        if prev_button.clicked() {
            update_player(PlayerAction::Prev, &token);
        }

        delay.delay_ms(50);
    }
}
