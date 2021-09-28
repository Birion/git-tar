use std::error::Error;

use clap::{App, Arg, ArgMatches};
use rand::prelude::*;
use serde::Deserialize;
use webbrowser;
use yansi::Paint;

#[derive(Deserialize, Debug, Clone)]
struct Genre {
    genre: String,
    examples: Vec<Song>,
}

#[derive(Deserialize, Debug, Clone)]
struct Song {
    title: String,
    artist: String,
    url: String,
}

fn get_matches() -> ArgMatches {
    App::new("git tar")
        .version("1.0.0")
        .author("Birion")
        .arg(
            Arg::new("youtube")
                .short('y')
                .long("youtube")
                .about("Opens the song on YouTube"),
        )
        .arg(
            Arg::new("genres")
                .short('g')
                .long("genres")
                .about("Picks only songs from the selected genre")
                .takes_value(true)
                .possible_values(&["acoustic", "rock", "folk", "classical", "electric"]),
        )
        .get_matches()
}

fn pick_one_genre(src: &str, genre: Option<&str>) -> Result<Genre, Box<dyn Error>> {
    let mut songs: Vec<Genre> = serde_yaml::from_str(src)?;
    match genre {
        None => {
            let mut rng = thread_rng();
            songs.shuffle(&mut rng);
            Ok(songs[0].clone())
        }
        Some(g) => {
            let songs: Vec<Genre> = songs.into_iter().filter(|x| &x.genre == g).collect();
            Ok(songs[0].clone())
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let matches = get_matches();
    let selected_genre = matches.value_of("genres");
    let songs_list = include_str!("songs.yaml");
    let mut rng = thread_rng();

    let mut genre = pick_one_genre(songs_list, selected_genre)?;

    genre.examples.shuffle(&mut rng);

    let random_song = &genre.examples[0];

    println!(
        "Let's try some {} guitar song. How about {} by {}?",
        genre.genre,
        Paint::green(&random_song.title),
        Paint::blue(&random_song.artist)
    );

    if matches.is_present("youtube") {
        let _ = webbrowser::open(&random_song.url)?;
    }

    Ok(())
}
