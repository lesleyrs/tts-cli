use std::{process, thread::sleep, time::Duration};

use arboard::Clipboard;
#[cfg(windows)]
use colored::control;
use colored::Colorize;
use tts::{Error, Features, Tts};

fn main() -> Result<(), Error> {
    #[cfg(windows)]
    control::set_virtual_terminal(true).ok();
    const FPS: u32 = 60;
    const FRAME_TIME: Duration = Duration::new(0, 1_000_000_000 / FPS);
    let mut clipboard = Clipboard::new().unwrap();
    let mut last = String::from("");
    let mut tts = Tts::default()?;
    if let Ok(voice) = tts.voice() {
        if let Some(v) = voice {
            println!(
                "Voice: {}, {:?}, {}\n",
                v.name(),
                v.gender().unwrap(),
                v.language()
            )
        }
    }
    let Features {
        utterance_callbacks,
        ..
    } = tts.supported_features();
    if utterance_callbacks {
        tts.on_utterance_begin(Some(Box::new(|utterance| {
            println!("Started speaking {:?}", utterance)
        })))?;
        tts.on_utterance_end(Some(Box::new(|utterance| {
            println!("Finished speaking {:?}", utterance)
        })))?;
    }
    loop {
        sleep(FRAME_TIME);
        match clipboard.get_text() {
            Ok(contents) => {
                if last != contents {
                    tts.speak(&contents, true)?;
                    println!("{}", contents.bright_yellow());
                    last = contents;
                }
            }
            Err(e) => {
                eprintln!("{}\n", e.to_string().bright_red());
                process::exit(1);
            }
        }
    }
}
