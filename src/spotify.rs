use esp_idf_svc::http::Method;

use crate::{request::request, util::remove_first_and_last};

pub enum PlayerAction {
    Pause,
    Play,
    Skip,
    Prev,
}

pub struct SpotifyToken {
    refresh_token: &'static str,
    client_auth: &'static str,
    api_token: String,
}

impl SpotifyToken {
    pub fn new(refresh_token: &'static str, client_auth: &'static str) -> Self {
        let mut token = Self {
            refresh_token,
            client_auth,
            api_token: String::new(),
        };
        token.regenerate();
        token
    }

    pub fn regenerate(&mut self) {
        let result = request(
            format!(
                "https://accounts.spotify.com/api/token?grant_type=refresh_token&refresh_token={}",
                self.refresh_token
            ),
            &[
                ("Content-Type", "application/x-www-form-urlencoded"),
                ("Authorization", self.client_auth),
            ],
            Method::Post,
        )
        .unwrap();

        let token_field = result.split(",").collect::<Vec<&str>>()[0];
        let token = token_field.split(":").collect::<Vec<&str>>()[1];

        self.api_token = remove_first_and_last(token.to_owned());
    }

    pub fn raw(&self) -> &str {
        &self.api_token
    }

    pub fn bearer(&self) -> String {
        let mut s = String::from("Bearer ");
        s.push_str(self.raw());
        s
    }
}

pub fn update_player(action: PlayerAction, token: &SpotifyToken) {
    let (endpoint, method) = match action {
        PlayerAction::Play => ("play", Method::Put),
        PlayerAction::Pause => ("pause", Method::Put),
        PlayerAction::Skip => ("next", Method::Post),
        PlayerAction::Prev => ("previous", Method::Post),
    };

    let url = String::from("https://api.spotify.com/v1/me/player/") + endpoint;
    println!(
        "\n\nURL: {}, \n\nTOKEN: {}\n\n",
        url,
        token.bearer().as_str()
    );
    if let Err(e) = request(url, &[("Authorization", token.bearer().as_str())], method) {
        println!("{}", e);
    }
}
