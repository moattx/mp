use rodio::source::{SineWave, Source};
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use std::{collections::HashMap, env, thread, time::Duration};
//use rodio::OutputStream;
use rodio::OutputStreamHandle;
use std::sync::mpsc;
use std::sync::Mutex;

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
    //let sink = Sink::try_new(&stream_handle).unwrap();
    //
    //

    thread::spawn(move || {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        let file = BufReader::new(File::open(song.clone()).unwrap());
        let source = Decoder::new(file).unwrap();
        let source_duration = source.total_duration().unwrap().as_secs();
        sink.append(source);

        let mut dur: u16 = 1;

        //let mut num = m.lock().unwrap();
        //num.append(source);
        loop {
            //check mutex
            dur = (dur + 1) - source_duration as u16;
            {
                let mut num = location.lock().unwrap();
                //*num = [dur, source.total_duration];
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


            //thread::sleep(Duration::from_millis(1000));
            //sink.sleep_until_end();
            //sink.sleep_until_end();
            //self.sink.append(source);
            //self.sink.sleep_until_end();
        }
        //break;
    });
}
