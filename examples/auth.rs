use rspotify::spotify::client::Spotify;
use rspotify::spotify::oauth2::{SpotifyClientCredentials, SpotifyOAuth};

fn main() {
    // Set client_id and client_secret in .env file or
    // export CLIENT_ID="your client_id"
    // export CLIENT_SECRET="secret"
    // export REDIRECT_URI=your-direct-uri

    // Or set client_id, client_secret,redirect_uri explictly
    // let oauth = SpotifyOAuth::default()
    //     .client_id("this-is-my-client-id")
    //     .client_secret("this-is-my-client-secret")
    //     .redirect_uri("http://localhost:8888/callback")
    //     .build();

    let mut oauth = SpotifyOAuth::default()
        .build();
    if let Some(token_info) = rspotify_hyper::get_token_hyper(&mut oauth) {
        let client_credential = SpotifyClientCredentials::default()
            .token_info(token_info)
            .build();
        let spotify = Spotify::default()
            .client_credentials_manager(client_credential)
            .build();

        dbg!(spotify);
    }
}