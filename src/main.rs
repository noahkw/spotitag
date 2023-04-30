use rspotify::clients::BaseClient;
use rspotify::model::{AlbumId, FullAlbum};
use rspotify::{ClientCredsSpotify, ClientResult, Credentials};

use std::io;

fn main() {
    let creds = Credentials::from_env().unwrap();
    let spotify = ClientCredsSpotify::new(creds);

    spotify.request_token().unwrap();

    let album: FullAlbum = loop {
        let meteora_20th = AlbumId::from_uri("spotify:album:3Q9wXhEAX7NYCPP0hxIuDz").unwrap();
        let album: ClientResult<FullAlbum> = spotify.album(meteora_20th);

        match album {
            Ok(album) => break album,
            Err(_) => {
                let mut user_decision = String::new();
                println!("Do you want to try again? y/n");
                io::stdin()
                    .read_line(&mut user_decision)
                    .expect("wow, can't even enter text");

                if user_decision == "y" {
                    continue;
                } else {
                    panic!("crash it 😭😭😭😭");
                }
            }
        };
    };

    println!("{:?}", album.tracks.items[1].name);
}
