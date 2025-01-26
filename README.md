# ESP32 Spotify Controller
This is a desktop spotify controller I am developing on an esp32-wroom-32e. 

## Installation
Follow the installation guide found [here](https://docs.esp-rs.org/book/installation/index.html) and don't forget to install the std development requirements.

If you are using a different controller for this, you might need to adjust some of the settings, or generate a std template for your controller following [this](https://docs.esp-rs.org/book/writing-your-own-application/generate-project/index.html) guide.

If you generated your own template, make sure you copy over `src`, and the dependencies and build dependencies in `cargo.toml`

Also, create a `cfg.toml` in the root of the project, and add the following. Remember to replace the values with your own!

```toml
wifi_ssid="YOUR_WIFI_SSID"
wifi_psk="YOUR_WIFI_PASSWORK"
refresh_token="YOUR_SPOTIFY_REFRESH_TOKEN"

# client_auth should be "Basic " + (your_spotify_client_id:your_spotify_client_secret).toBase64()
# example "Basic MTIzNDU2Nzg5MDEyMzQ1Njc4OTAxMjM0NTY3ODkwMTI6MTIzNDU2Nzg5MDEyMzQ1Njc4OTAxMjM0NTY3ODkwMQ=="
client_auth="YOUR_CLIENT_AUTH"
```

Once everything is installed, you should now be able to run `cargo run --release` and everything should work properly\
Note: I have not tested this myself on another machine or controller, I only included the steps I used to create my own project

## Pins
This is how I have my pins setup. If you want to change it, it is very easy to change the pins in `main.rs`.

| Function | Mode | Pin |
|----------|------|-----|
|Play Song|INPUT|18|
|Pause Song|INPUT|5|
|Skip Song|INPUT|19|
|Previous Song|INPUT|17|
|Wifi Status LED|OUTPUT|16|