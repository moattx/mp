use crate::ui::ui;
//use crate::audio::Audio;
use crate::audio::audio_play;
use crate::audio::send_duration;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::prelude::CrosstermBackend;
use ratatui::terminal;
use ratatui::widgets::ListState;
use std::fs;
use std::io;
use std::time::Duration;

const PLAYLIST_PATH: &str = "/home/moatx/Music";

// XXX: do not try the Option usize

// XXX: Do not rename as 'List' -- it's taken.
#[derive(PartialEq)]
pub enum Lists {
    Playlists,
    Songs,
}

pub struct App {
    pub running: bool,
    pub playlist: ListState,
    pub playlist_items: Vec<String>,
    pub song: ListState,
    pub song_items: Vec<String>,
    pub list: Lists,
    pub item: usize,
    pub divide_list: bool,
    //pub saved_item: Option<usize>,
    pub saved_item: usize,
    //pub audio: Audio,
}

impl App {
    pub fn new() -> Self {
        Self {
            running: true,
            playlist: <ListState as std::default::Default>::default(),
            playlist_items: get_playlists(),
            song: <ListState as std::default::Default>::default(),
            song_items: vec!["".to_string()],
            list: Lists::Playlists,
            item: 0,
            divide_list: false,
            //saved_item: None,
            saved_item: 0,
            //audio: Audio::new(),
        }
    }

    pub fn run(
        &mut self,
        mut terminal: terminal::Terminal<CrosstermBackend<io::Stdout>>,
    ) -> io::Result<()> {
        // draw
        while self.running {
            terminal.draw(|frame| ui(frame, self))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // XXX: it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                match key_event.code {
                    KeyCode::Char('q') => self.toggle_running(),
                    KeyCode::Char('j') => self.go_down(),
                    KeyCode::Char('k') => self.go_up(),
                    KeyCode::Tab => self.switch_lists(),
                    KeyCode::Enter => self.play_song(),
                    KeyCode::Right => self.seek_forward(),
                    _ => (),
                }
            }
            _ => {}
        };
        Ok(())
    }
    pub fn toggle_running(&mut self) {
        self.running = !self.running;
    }

    pub fn play_song(&mut self) {
        if self.list == Lists::Songs {
            audio_play(
                PLAYLIST_PATH.to_owned()
                    + "/"
                    + &self.playlist_items[self.saved_item]
                    + "/"
                    + &self.song_items[self.item],
            );
        }
    }
    pub fn seek_forward(&mut self) {
        send_duration(1);
    }

    pub fn switch_lists(&mut self) {
        match self.list {
            Lists::Playlists => {
                //match self.saved_item {
                //    None => None,
                //    Some(s) => Some(self.item = s),
                //}.expect("cant set self.item");
                //self.item = self.saved_item;

                // Save the currently selected item for Lists::Songs
                //self.saved_item = Some(self.item);
                self.saved_item = self.item;
                self.item = 0;

                self.list = Lists::Songs;
            }
            Lists::Songs => {
                // Update self.item for **switched** List
                //match self.saved_item {
                //    None => None,
                //    Some(s) => Some(self.item = s),
                //}.expect("cant set self.item");
                self.item = self.saved_item;

                // Save the currently selected item for Lists::Songs
                //self.saved_item = Some(self.item);
                //self.saved_item = self.item;
                self.list = Lists::Playlists;
            }
        }
    }

    pub fn go_up(&mut self) {
        if self.item == 0 {
            self.item = 0;
        } else {
            self.item -= 1;
        }
        self.select_item();
    }

    pub fn go_down(&mut self) {
        let max_len = match self.list {
            Lists::Playlists => self.playlist_items.len() - 1,
            Lists::Songs => self.song_items.len() - 1,
        };
        if self.item == max_len {
            self.item = max_len;
        } else {
            self.item += 1;
        }

        self.select_item();
    }
    pub fn select_item(&mut self) {
        match self.list {
            Lists::Playlists => {
                self.playlist.select(Some(self.item));
                //println!("{}", PLAYLIST_PATH.to_owned() + "/" + &self.playlist_items[self.item]);
                self.song_items =
                    get_songs(PLAYLIST_PATH.to_owned() + "/" + &self.playlist_items[self.item]);
                // if Lists::Playlists then Lists::Songs
                // switch to Lists::Songs, as there is a playlist selected
                //self.list = Lists::Songs;
                // divide list, as there is a playlist selected
                if self.divide_list != true {
                    self.divide_list = true;
                }
            }
            Lists::Songs => {
                self.song.select(Some(self.item));
            }
        }
    }
}

pub fn get_songs(playlist: String) -> Vec<String> {
    let mut songs: Vec<String> = Vec::new();
    let playlist_dir = fs::read_dir(playlist).unwrap();
    for song in playlist_dir {
        let name = song.unwrap().path().into_os_string().into_string().unwrap();
        let parts = name.split('/');
        let collection: Vec<&str> = parts.collect();
        songs.push(collection.last().unwrap().to_string());
    }
    songs
}

fn get_playlists() -> Vec<String> {
    let mut playlists: Vec<String> = Vec::new();
    let playlists_dir = fs::read_dir(PLAYLIST_PATH).unwrap();
    for playlist in playlists_dir {
        let name = playlist
            .unwrap()
            .path()
            .into_os_string()
            .into_string()
            .unwrap();

        if !fs::metadata(&name).unwrap().is_dir() {
            continue;
        }
        let parts = name.split('/');
        let collection: Vec<&str> = parts.collect();
        playlists.push(collection.last().unwrap().to_string());
    }
    playlists
}
