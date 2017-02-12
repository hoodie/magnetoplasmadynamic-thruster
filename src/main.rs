#![feature(plugin)]
#![feature(field_init_shorthand)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate mpd;
extern crate lazy_static;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

// steps:
// 1. show currently playing on start page
// 2. show playlist
// 3. pause/play

use std::io;
use std::net::TcpStream;
use std::convert::From;
use std::collections::BTreeMap;

//use rocket::response::NamedFile;
use rocket::response::Redirect;
use rocket_contrib::Template;
use rocket_contrib::JSON;

use mpd::Client as MpdClient;
use mpd::song::Song as MpdSong;

mod mpd_wrappers {
    use super::*;
    #[derive(Serialize)]
    pub struct Song {
        /// filename
        pub file: String,
        /// name (for streams)
        pub name: Option<String>,
        /// title
        pub title: Option<String>,
        pub tags: BTreeMap<String, String>,
        pub running: bool
    }

    impl From<MpdSong> for Song {
        fn from(song:MpdSong) -> Song {
            Song {
                file: song.file,
                name: song.name,
                title: song.title,
                tags: song.tags,
                running: false,
            }
        }
    }
}


use mpd_wrappers::Song;


fn get_client() -> MpdClient {
    let stream = TcpStream::connect(option_env!("MPD_HOST").unwrap_or("localhost:6600")).unwrap();
    let mut client = MpdClient::new(stream).unwrap();
    if let Some(password) = option_env!("MPD_PW") {
        client.login(password).unwrap();
    }
    client
}

#[derive(Serialize)]
struct PlayerStatus{
    song: Option<Song>,
    queue: Vec<Song>
}

impl PlayerStatus {
    pub fn current() -> PlayerStatus {
        let mut client = get_client();
        let song = client.currentsong();
        //println!("{:?}", song);
        let song:Option<Song> = song
            .unwrap(/*Result*/)
            .map(|song| Song::from(song));
        println!("{}", serde_json::ser::to_string(&song).unwrap());

        let queue = client.queue()
                          .unwrap()
                          .into_iter()
                          //.map(Into::into)
                          .map(|s| Song::from(s))
                          .map(|mut s| {
                              if let Some(ref song) = song {
                                  s.running = song.title == s.title
                              }
                              s
                          })
                          .collect::<Vec<_>>();
        PlayerStatus{
            song,
            queue
        }
    }
}

#[get("/current")]
fn current() -> Template {
    Template::render("index", &PlayerStatus::current())
}

#[get("/current.js")]
fn current_js() -> JSON<PlayerStatus> {
    JSON(PlayerStatus::current())
}

#[post("/prev")]
fn prev() -> Redirect {
    let mut client = get_client();
    client.prev().unwrap();
    Redirect::to("/")
}

#[post("/next")]
fn next() -> io::Result<Redirect> {
    let mut client = get_client();
    client.next()
        .map_err(|_|io::Error::new(io::ErrorKind::Other, "cannot go further"))?;
    Ok(Redirect::to("/"))
}

#[post("/play")]
fn play() -> Redirect {
    let mut client = get_client();
    client.play().unwrap();
    Redirect::to("/")
}

#[post("/pause")]
fn pause() -> Redirect {
    let mut client = get_client();
    client.pause(true).unwrap();
    Redirect::to("/")
}

#[get("/")]
fn home() -> Redirect {
    //fn home() -> io::Result<NamedFile> {
    //NamedFile::open("static/index.html")
    Redirect::to("/current")
}

fn main() {
    rocket::ignite()
        .mount("/", routes![home])
        .mount("/", routes![play,pause,next,prev])
        .mount("/", routes![current, current_js])
        .launch();
}
