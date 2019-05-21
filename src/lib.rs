use futures::future::Future;
use hyper::service::service_fn_ok;
use hyper::Body;
use hyper::Response;
use hyper::Server;
use rspotify::spotify::oauth2::SpotifyOAuth;
use rspotify::spotify::oauth2::TokenInfo;
use rspotify::spotify::util::generate_random_string;
use std::sync::mpsc::channel;
use std::thread;

pub fn get_token_hyper(spotify_oauth: &mut SpotifyOAuth) -> Option<TokenInfo> {
    match spotify_oauth.get_cached_token() {
        Some(token_info) => Some(token_info),
        None => {
            let (tx, rx) = channel::<TokenInfo>();

            let state = generate_random_string(16);
            let auth_url = spotify_oauth.get_authorize_url(Some(&state), None);
            let (_tx_stop, rx_stop) = futures::sync::oneshot::channel::<()>();

            let spotify_oauth = spotify_oauth.clone();
            thread::spawn(move || {
                let tx = tx.clone();

                webbrowser::open(&auth_url).unwrap();

                let addr = ([127, 0, 0, 1], 8888).into();

                let new_svc = move || {
                    let spotify_oauth = spotify_oauth.clone();
                    let tx = tx.clone();

                    service_fn_ok(move |req| {
                        let tx = tx.clone();

                        let mut uri = req.uri().to_string();
                        let path = req.uri().path();

                        let token = spotify_oauth
                            .parse_response_code(&mut uri)
                            .and_then(|code| spotify_oauth.get_access_token(&code));

                        let success = "<html><head></head><body><script>window.close();</script></body></html>";
                        let failure = "<html><head></head><body>Invalid path</body></html>";

                        let resp = match token {
                            Some(token) => {
                                if path == "/callback" {
                                    tx.send(token).unwrap();
                                    success
                                } else {
                                    failure
                                }
                            }
                            _ => failure,
                        };

                        Response::new(Body::from(resp))
                    })
                };

                let server = Server::bind(&addr)
                    .serve(new_svc)
                    .with_graceful_shutdown(rx_stop)
                    .map_err(|e| eprintln!("server error: {}", e));

                // Run this server for... forever!
                hyper::rt::run(server);
            });

            rx.recv().ok()
        }
    }
}
