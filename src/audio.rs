use rodio::source::{SineWave, Source};
use rodio::OutputStreamHandle;
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use std::sync::mpsc;
use std::sync::Mutex;
use std::{collections::HashMap, env, thread, time::Duration};

static m: Mutex<Duration> = Mutex::new(Duration::ZERO);

static location: Mutex<u16> = Mutex::new(0);

pub fn get_location() -> u16 {
    let mut num = location.lock().unwrap();
    *num
}

pub fn send_duration(duration: u64) {
    let mut num = m.lock().unwrap();
    *num = Duration::from_secs(duration);
}

// send a message to the thread somehow, maybe via a mutex? It will prediacly check the mutex for
// updates every second in a loop

pub fn audio_play(song: String) {
    thread::spawn(move || {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        let file = BufReader::new(File::open(song.clone()).unwrap());
        let source = Decoder::new(file).unwrap();
        let source_duration = source.total_duration().unwrap().as_secs();
        sink.append(source);

        let mut dur: u16 = 1;

        loop {
            dur = (dur + 1) - source_duration as u16;
            {
                let mut num = location.lock().unwrap();
                *num = dur;
            }
            {
                let num = m.lock().unwrap();
                if *num != Duration::ZERO {
                    sink.try_seek(*num).unwrap();
                }
            }

            if sink.empty() {
                return;
            }
        }
    });
}
